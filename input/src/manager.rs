use crate::mapping::{InputMapping, Mapping};
use crate::{ButtonId, Guesture, ModifierFilterMask, ModifierId, State};
use std::mem;

pub struct Manager {
    time: u128,
    mapping: Mapping,
    guestures: Vec<(String, Box<Guesture>)>,
    state: State,
    previous_state: State,
}

impl Manager {
    fn now() -> u128 {
        use std::time::SystemTime;
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros()
    }

    pub fn new() -> Manager {
        Manager {
            time: 0,
            mapping: Mapping::new(),
            guestures: Vec::new(),
            state: State::new(),
            previous_state: State::new(),
        }
    }

    pub fn add_guesture<S: ToString>(&mut self, name: S, guesture: Box<Guesture>) {
        let name: String = name.to_string();
        assert!(self.guestures.iter().find(|v| v.0 == name).is_none());
        self.guestures.push((name, guesture));
    }

    pub fn get_guesture<S: ToString>(&self, name: S) -> Option<&Guesture> {
        let name = name.to_string();
        self.guestures
            .iter()
            .find_map(|v| if v.0 == name { Some(v.1.as_ref()) } else { None })
    }

    pub fn get_guesture_mut<S: ToString>(&mut self, name: S) -> Option<&mut Guesture> {
        let name = name.to_string();
        let id = self.guestures.iter().position(|v| v.0 == name);
        match id {
            None => None,
            Some(i) => Some(self.guestures[i].1.as_mut()),
        }
    }

    pub fn add_modifier_mapping(&mut self, input_event: InputMapping, modifier_id: ModifierId) {
        self.mapping.add_modifier_mapping(input_event, modifier_id);
    }

    pub fn add_button_mapping(
        &mut self,
        input_event: InputMapping,
        input_modifiers: Option<ModifierFilterMask>,
        button_id: ButtonId,
        sensitivity: f32,
    ) {
        self.mapping
            .add_button_mapping(input_event, input_modifiers, button_id, sensitivity);
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn prepare(&mut self) {
        self.time = Self::now();
        mem::swap(&mut self.previous_state, &mut self.state);
        self.state.prepare(&self.previous_state, self.time);
    }

    pub fn update(&mut self) {
        for (_, ref mut guesture) in self.guestures.iter_mut() {
            guesture.on_update(&self.previous_state, &mut self.state);
        }
    }

    pub fn handle_winit_events(&mut self, event: &winit::Event) {
        use winit::{DeviceEvent, ElementState, Event, WindowEvent};

        // keyboard, button, and mouse position is handled through window events
        // mouse "delta" movement is handled through device

        match *event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                // handling mapped keyboards
                log::trace!("winit key: {:?}", input);

                if let Some(modifier_id) = self.mapping.map_winit_key_to_modifier(&input) {
                    self.state
                        .set_modifier(modifier_id, input.state == ElementState::Pressed, false);
                }

                if let Some((button_id, modifier_mask, sensitivity)) = self.mapping.map_winit_key_to_button(&input) {
                    let value = if input.state == ElementState::Pressed { 1. } else { 0. };
                    log::trace!(
                        "mapped winit key: {:?} to axis: {:?},{},{}",
                        input,
                        button_id,
                        value,
                        value * sensitivity
                    );
                    self.state.set_button(button_id, modifier_mask, value * sensitivity, false);
                }
            }
            Event::DeviceEvent {
                device_id,
                event: DeviceEvent::Motion { axis, value },
            } => {
                log::trace!("winit dev motion: {:?},{:?},{:?}", device_id, axis, value);

                if let Some(modifier_id) = self.mapping.map_winit_axis_to_modifier(device_id, axis) {
                    self.state.set_modifier(modifier_id, true, true);
                }

                if let Some((button_id, modifier_mask, sensitivity)) = self.mapping.map_winit_axis_to_button(device_id, axis) {
                    let value = value as f32;
                    log::trace!(
                        "mapping winit axis: {:?},{:?},{:?} to axis: {:?},{},{}",
                        device_id,
                        axis,
                        value,
                        button_id,
                        value,
                        value * sensitivity
                    );
                    self.state.set_button(button_id, modifier_mask, value * sensitivity, true);
                }
            }
            Event::DeviceEvent {
                device_id,
                event: DeviceEvent::Added,
            } => {
                //todo: mapping - add/remove known devices
                log::trace!("dev added: {:?}", device_id);
            }
            _ => {}
        };
    }

    pub fn handle_gil_events(&mut self, event: &gilrs::Event) {
        use gilrs::{Event, EventType};

        let Event { id, event, .. } = event;
        match event {
            EventType::AxisChanged(axis, value, ..) => {
                log::trace!("gil axis {:?},{:?},{:?}", id, axis, value);

                if let Some(modifier_id) = self.mapping.map_gil_axis_to_modifier(*id, *axis) {
                    self.state.set_modifier(modifier_id, *value != 0., false);
                }

                if let Some((button_id, modifier_mask, sensitivity)) = self.mapping.map_gil_axis_to_button(*id, *axis) {
                    log::trace!(
                        "mapping gil axis: {:?},{:?} to axis: {:?},{},{}",
                        id,
                        axis,
                        button_id,
                        value,
                        value * sensitivity
                    );
                    self.state.set_button(button_id, modifier_mask, value * sensitivity, false);
                }
            }
            EventType::ButtonChanged(button, value, ..) => {
                log::trace!("gil button {:?},{:?}", button, value);
            }
            _ => {}
        }
    }
}

impl Default for Manager {
    fn default() -> Manager {
        Manager::new()
    }
}
