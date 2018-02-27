use std::thread;
use std::sync;
use std::sync::{Arc, Weak};
use std::sync::mpsc;
use core::*;
use framework::*;

/// Window implementation for opengl
pub type PlatformWindow = GLWindow;

/// Window handler for opengl
pub struct PlatformWindowHandler {
    queue: mpsc::Sender<WindowCmd>,
    control: Weak<()>,
    _join: Option<thread::JoinHandle<()>>,
}

impl WindowHandler<PlatformEngine> for PlatformWindowHandler {
    fn is_closed(&self) -> bool {
        self.control.upgrade().is_none()
    }

    fn send_command(&self, cmd: WindowCommand) {
        self.queue.send(WindowCmd::Async(cmd)).unwrap();
    }

    fn send_sync_command(&self, cmd: WindowCommand) {
        let barrier = sync::Arc::new(sync::Barrier::new(2));
        self.queue.send(WindowCmd::Sync(cmd, barrier.clone())).unwrap();
        barrier.wait();
    }

    fn close(&mut self) {
        match self.queue.send(WindowCmd::RequestClose) {
            _ => {}
        }
    }
}

impl Drop for PlatformWindowHandler {
    fn drop(&mut self) {
        self.close();
    }
}

impl WindowBuilder<PlatformEngine> for WindowSettings<GLExtraWindowSettings> {
    type WindowHandler = PlatformWindowHandler;
    type Window = PlatformWindow;

    fn build<Ctx, F>(&self, engine: &PlatformEngine, timeout: DispatchTimeout, ctx: Ctx, callback: F) -> Result<Self::WindowHandler, Error>
        where
            F: 'static + Send + Fn(&mut Self::Window, &mut Ctx, &WindowCommand),
            Ctx: 'static + Send {
        let (tx, rx) = mpsc::channel::<WindowCmd>();
        let win = GLWindow::new(self, engine.platform(), tx.clone())?;
        let working = Arc::new(());
        let control = Arc::downgrade(&working);
        let join = match timeout {
            DispatchTimeout::Infinite =>
                thread::spawn(move || {
                    let mut ctx = ctx;
                    let mut win = win;
                    let working = working;
                    while !win.is_closed() {
                        match rx.recv() {
                            Ok(WindowCmd::Sync(data, barrier)) => {
                                win.pre_hook(&data);
                                callback(&mut win, &mut ctx, &data);
                                win.post_hook(&data);
                                barrier.wait();
                            }
                            Ok(WindowCmd::Async(data)) => {
                                win.pre_hook(&data);
                                callback(&mut win, &mut ctx, &data);
                                win.post_hook(&data);
                            }
                            Ok(WindowCmd::RequestClose) => {
                                win.close();
                            }
                            Err(mpsc::RecvError) => {
                                break;
                            }
                        }
                    }
                    drop(working);
                }),

            DispatchTimeout::Immediate =>
                thread::spawn(move || {
                    let mut ctx = ctx;
                    let mut win = win;
                    let working = working;
                    while !win.is_closed() {
                        match rx.try_recv() {
                            Ok(WindowCmd::Sync(data, barrier)) => {
                                win.pre_hook(&data);
                                callback(&mut win, &mut ctx, &data);
                                win.post_hook(&data);
                                barrier.wait();
                            }
                            Ok(WindowCmd::Async(data)) => {
                                win.pre_hook(&data);
                                callback(&mut win, &mut ctx, &data);
                                win.post_hook(&data);
                            }
                            Ok(WindowCmd::RequestClose) => {
                                win.close();
                            }
                            Err(mpsc::TryRecvError::Empty) => {
                                callback(&mut win, &mut ctx, &WindowCommand::Tick);
                            }
                            Err(mpsc::TryRecvError::Disconnected) => {
                                break;
                            }
                        }
                    }
                    drop(working);
                }),

            DispatchTimeout::Time(duration) =>
                thread::spawn(move || {
                    let mut ctx = ctx;
                    let mut win = win;
                    let working = working;
                    while !win.is_closed() {
                        match rx.recv_timeout(duration) {
                            Ok(WindowCmd::Sync(data, barrier)) => {
                                win.pre_hook(&data);
                                callback(&mut win, &mut ctx, &data);
                                win.post_hook(&data);
                                barrier.wait();
                            }
                            Ok(WindowCmd::Async(data)) => {
                                win.pre_hook(&data);
                                callback(&mut win, &mut ctx, &data);
                                win.post_hook(&data);
                            }
                            Ok(WindowCmd::RequestClose) => {
                                win.close();
                            }
                            Err(mpsc::RecvTimeoutError::Timeout) => {
                                callback(&mut win, &mut ctx, &WindowCommand::Tick);
                            }
                            Err(mpsc::RecvTimeoutError::Disconnected) => {
                                break;
                            }
                        }
                    }
                    drop(working);
                }),
        };

        Ok(PlatformWindowHandler {
            queue: tx,
            control: control,
            _join: Some(join),
        })
    }
}