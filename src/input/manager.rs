use crate::input::{GuestureHandler, GuestureResponse, State};

pub struct Manager {
    time: u128,
    state: State,
    scope: Vec<String>,
    current_scope: String,
    guestures: Vec<(String, String, Box<GuestureHandler>)>,
}

impl Manager {
    fn now() -> u128 {
        use std::time::SystemTime;
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros()
    }

    pub fn new() -> Manager {
        Manager {
            time: 0,
            scope: Vec::new(),
            current_scope: String::new(),
            guestures: Vec::new(),
            state: State::new(),
        }
    }

    pub fn add_guesture<S1: ToString, S2: ToString>(&mut self, name: S1, scope: S2, guesture: Box<GuestureHandler>) {
        let name: String = name.to_string();
        let scope: String = scope.to_string();
        assert!(self.guestures.iter().find(|v| v.0 == name).is_none());
        self.guestures.push((name, scope, guesture));
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn push_scope<S: ToString>(&mut self, s: S) {
        self.scope.push(s.to_string());
        self.current_scope = self.scope.join(".");
    }

    pub fn pop_scope(&mut self) -> String {
        let s = self.scope.pop().unwrap();
        self.current_scope = self.scope.join(".");
        s
    }

    pub fn get_scope(&self) -> &str {
        &self.current_scope
    }

    fn check_scope(scope: &str, guesture: &str) -> bool {
        //todo
        true
    }

    pub fn prepare(&mut self) {
        self.state.auto_reset_joystick();

        for (_, ref scope, ref mut guesture) in self.guestures.iter_mut() {
            if !Self::check_scope(&self.current_scope, scope) {
                return;
            }

            guesture.on_prepare(self.time, &mut self.state);
        }
    }

    pub fn update(&mut self) {
        for (_, ref scope, ref mut guesture) in self.guestures.iter_mut() {
            if !Self::check_scope(&self.current_scope, scope) {
                return;
            }

            guesture.on_update(self.time, &mut self.state);
        }
    }

    fn map_winit_joystic(&self, device_id: &winit::DeviceId, axis: u32) -> Option<(u32, f32)> {
        Some((axis, 0.1))
    }

    pub fn handle_winit_events(&mut self, event: &winit::Event) {
        use winit::{DeviceEvent, Event};

        match *event {
            /*Event::WindowEvent {} => {

            }*/
            Event::DeviceEvent {
                device_id,
                event: DeviceEvent::Motion { axis, value },
            } => {
                log::trace!("mapping winit joystick: dev:{:?} axis:{}", device_id, axis);
                if let Some((axis_id, sensitivity)) = self.map_winit_joystic(&device_id, axis) {
                    let value = value as f32;
                    log::trace!("value: raw:{}, scaled:{}", value, value * sensitivity);
                    self.on_joystick(axis_id, value * sensitivity, true);
                }
            }
            _ => {}
        };
    }

    fn map_gil_joystic(&self, device_id: &gilrs::GamepadId, axis: &gilrs::ev::Axis) -> Option<(u32, f32)> {
        Some((0, 1.))
    }

    pub fn handle_gil_events(&mut self, event: &gilrs::Event) {
        use gilrs::{Event, EventType::AxisChanged};

        let Event { id, event, .. } = event;
        match event {
            AxisChanged(axis, value, ..) => {
                log::trace!("mapping gil joystick: dev:{:?} axis:{:?}", id, axis);
                if let Some((axis_id, sensitivity)) = self.map_gil_joystic(id, axis) {
                    log::trace!("value: raw:{}, scaled:{}", value, value * sensitivity);
                    self.on_joystick(axis_id, value * sensitivity, false);
                }
            }
            _ => {}
        }
    }

    fn on_joystick(&mut self, axis_id: u32, value: f32, auto_reset: bool) {
        for (_, ref scope, ref mut guesture) in self.guestures.iter_mut() {
            if !Self::check_scope(&self.current_scope, scope) {
                return;
            }

            if guesture.on_joystick(self.time, &mut self.state, axis_id, value) == GuestureResponse::Consumed {
                return;
            }
        }

        self.state.set_joystick(axis_id, value, auto_reset);
    }
}
