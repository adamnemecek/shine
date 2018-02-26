use core::*;
use framework::*;
use std::thread;
use std::sync;
use std::sync::mpsc;


/// Window implementation for opengl
pub type PlatformWindow = GLWindow;

/// Window handler for opengl
pub struct PlatformWindowHandler {
    queue: mpsc::Sender<WindowCmd>,
    join: Option<thread::JoinHandle<()>>,
}

impl WindowHandler<PlatformEngine> for PlatformWindowHandler {
    fn is_closed(&self) -> bool {
        self.join.is_none()
    }

    fn send_command(&self, cmd: WindowCommand) {
        self.queue.send(WindowCmd::Async(cmd)).unwrap();
    }

    fn send_sync_command(&self, cmd: WindowCommand) {
        let barrier = sync::Arc::new(sync::Barrier::new(2));
        self.queue.send(WindowCmd::Sync(cmd, barrier.clone())).unwrap();
        barrier.wait();
    }

    fn wait_close(&mut self) {
        self.queue.send(WindowCmd::RequestClose).unwrap();
        if let Some(join) = self.join.take() {
            join.join().unwrap();
        }
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
        let join =
            match timeout {
                DispatchTimeout::Infinite =>
                    thread::spawn(move || {
                        let mut ctx = ctx;
                        let mut win = win;
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
                                    panic!("Window connection closed by main thread");
                                }
                            }
                        }
                    }),

                DispatchTimeout::Immediate =>
                    thread::spawn(move || {
                        let mut ctx = ctx;
                        let mut win = win;
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
                                    panic!("Window connection closed by main thread");
                                }
                            }
                        }
                    }),

                DispatchTimeout::Time(duration) =>
                    thread::spawn(move || {
                        let mut ctx = ctx;
                        let mut win = win;
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
                                    panic!("Window connection closed by main thread");
                                }
                            }
                        }
                    }),
            };

        Ok(PlatformWindowHandler {
            queue: tx,
            join: Some(join),
        })
    }
}