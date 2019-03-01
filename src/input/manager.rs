use crate::input::{GuestureHandler, GuestureResponse, State};
use std::collections::HashMap;

pub struct Manager {
    time: u64,
    state: State,
    guestures: Vec<(String, Box<GuestureHandler>)>,
}

impl Manager {
    fn now() -> u64 {
        0
    }

    pub fn new() -> Manager {
        Manager {
            time: Manager::now(),
            state: State::new(),
            guestures: Vec::new(),
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn add_guesture<S: ToString>(&mut self, name: S, guesture: Box<GuestureHandler>) {
        let name: String = name.to_string();
        assert!(self.guestures.iter().find(|v| v.0 == name).is_none());
        self.guestures.push((name, guesture));
    }

    fn map_winit_joystic(&self, id: u32, axis: u32) -> u32 {
        axis
    }

    pub fn handle_winit_events(&mut self, event: &winit::Event) {
        use winit::{DeviceEvent, Event};

        match *event {
            Event::DeviceEvent {
                deviceId,
                event: DeviceEvent::Motion { axis, value },
            } => {
                if let Some((axis_id, sensitivity)) = self.map_winit_joystic(deviceId, axis) {
                    self.on_joystick(axis_id, value * sensitivity);
                }
            }
            _ => {}
        };
    }

    fn map_gil_joystic(&self, id: u32, axis: u32) -> u32 {
        axis
    }

    pub fn handle_gil_events(&mut self, event: &gilrs::Event) {
        use gilrs::{Event, EventType::AxisChanged};

        let Event { id, event, time } = event;
        match event {
            AxisChanged(axis, value, ..) => {
                if let Some((axis_id, sensitivity)) = self.map_gil_joystic(id, axis) {
                    self.on_joystick(axis_id, value * sensitivity);
                }
            }
            _ => {}
        }
    }

    fn on_joystick(&mut self, axis_id: u32, value: f32) {
        for (_, ref mut guesture) in self.guestures.iter_mut() {
            if guesture.on_joystick(axis_id, value) == GuestureResponse::Consumed {
                return;
            }
        }

        self.state.set_joystick(axis_id, value);
    }
}
