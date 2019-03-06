use crate::input::mapping::{InputMapping, Mapping};
use crate::input::{ButtonId, GuestureHandler, ModifierFilter, ModifierFilterMask, ModifierId, State};
use std::mem;

pub struct Manager {
    time: u128,
    mapping: Mapping,
    guestures: Vec<(String, Box<GuestureHandler>)>,
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

    pub fn add_guesture<S1: ToString, S2: ToString>(&mut self, name: S1, guesture: Box<GuestureHandler>) {
        let name: String = name.to_string();
        assert!(self.guestures.iter().find(|v| v.0 == name).is_none());
        self.guestures.push((name, guesture));
    }

    pub fn add_modifier_mapping(&mut self, input_event: InputMapping, modifier_id: ModifierId) {
        self.mapping.add_modifier_mapping(input_event, modifier_id);
    }

    pub fn add_button_mapping(
        &mut self,
        input_event: InputMapping,
        input_modifiers: Option<&[(ModifierId, ModifierFilter)]>,
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

                if let Some(modifier_id) = self.mapping.map_winit_axis_to_modifier(&device_id, axis) {
                    self.state.set_modifier(modifier_id, true, true);
                }

                if let Some((button_id, modifier_mask, sensitivity)) = self.mapping.map_winit_axis_to_button(&device_id, axis) {
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
        use gilrs::{Event, EventType::AxisChanged};

        let Event { id, event, .. } = event;
        match event {
            AxisChanged(axis, value, ..) => {
                log::trace!("gil axis {:?},{:?},{:?}", id, axis, value);

                if let Some(modifier_id) = self.mapping.map_gil_axis_to_modifier(id, axis) {
                    self.state.set_modifier(modifier_id, *value != 0., false);
                }

                if let Some((button_id, modifier_mask, sensitivity)) = self.mapping.map_gil_axis_to_button(id, axis) {
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
            _ => {}
        }
    }
}
