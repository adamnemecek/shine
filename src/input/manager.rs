use crate::input::mapping::{InputMapping, Mapping};
use crate::input::{AxisId, GuestureHandler, ModifierFilter, ModifierFilterMask, ModifierId, State};

pub struct Manager {
    time: u128,
    state: State,
    scope: Vec<String>,
    current_scope: String,
    mapping: Mapping,
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
            mapping: Mapping::new(),
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

    pub fn add_modifier_mapping(&mut self, input_event: InputMapping, modifier_id: ModifierId) {
        self.mapping.add_modifier_mapping(input_event, modifier_id);
    }

    pub fn add_axis_mapping(
        &mut self,
        input_event: InputMapping,
        input_modifiers: Option<&[(ModifierId, ModifierFilter)]>,
        axis_id: AxisId,
        sensitivity: f32,
    ) {
        self.mapping
            .add_axis_mapping(input_event, input_modifiers, axis_id, sensitivity);
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

    fn check_scope(manager_scope: &str, guesture_scope: &str) -> bool {
        manager_scope == guesture_scope
    }

    pub fn prepare(&mut self) {
        self.time = Self::now();
        self.state.time = self.time;
        self.state.autoreset_modifiers();
        self.state.autoreset_joystick();

        for (_, ref scope, ref mut guesture) in self.guestures.iter_mut() {
            if !Self::check_scope(&self.current_scope, scope) {
                return;
            }

            guesture.on_prepare(&mut self.state);
        }
    }

    pub fn update(&mut self) {
        for (_, ref scope, ref mut guesture) in self.guestures.iter_mut() {
            if !Self::check_scope(&self.current_scope, scope) {
                return;
            }

            guesture.on_update(&mut self.state);
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
                    self.on_modifier(modifier_id, input.state == ElementState::Pressed, false);
                }

                if let Some((axis_id, modifier_mask, sensitivity)) = self.mapping.map_winit_key_to_axis(&input) {
                    let value = if input.state == ElementState::Pressed { 1. } else { 0. };
                    log::trace!(
                        "mapped winit key: {:?} to axis: {:?},{},{}",
                        input,
                        axis_id,
                        value,
                        value * sensitivity
                    );
                    self.on_joystick(axis_id, modifier_mask, value * sensitivity, false);
                }
            }
            Event::DeviceEvent {
                device_id,
                event: DeviceEvent::Motion { axis, value },
            } => {
                log::trace!("winit dev motion: {:?},{:?},{:?}", device_id, axis, value);

                if let Some(modifier_id) = self.mapping.map_winit_axis_to_modifier(&device_id, axis) {
                    self.on_modifier(modifier_id, true, true);
                }

                if let Some((axis_id, modifier_mask, sensitivity)) = self.mapping.map_winit_axis_to_axis(&device_id, axis) {
                    let value = value as f32;
                    log::trace!(
                        "mapping winit axis: {:?},{:?},{:?} to axis: {:?},{},{}",
                        device_id,
                        axis,
                        value,
                        axis_id,
                        value,
                        value * sensitivity
                    );
                    self.on_joystick(axis_id, modifier_mask, value * sensitivity, true);
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
                    self.on_modifier(modifier_id, *value != 0., false);
                }

                if let Some((axis_id, modifier_mask, sensitivity)) = self.mapping.map_gil_axis_to_axis(id, axis) {
                    log::trace!(
                        "mapping gil axis: {:?},{:?} to axis: {:?},{},{}",
                        id,
                        axis,
                        axis_id,
                        value,
                        value * sensitivity
                    );
                    self.on_joystick(axis_id, modifier_mask, value * sensitivity, false);
                }
            }
            _ => {}
        }
    }

    fn on_modifier(&mut self, modifier_id: ModifierId, is_pressed: bool, auto_reset: bool) {
        self.state.set_modifier(modifier_id, is_pressed, auto_reset);
    }

    fn on_joystick(&mut self, axis_id: AxisId, modifier_mask: ModifierFilterMask, value: f32, auto_reset: bool) {
        self.state.set_joystick(axis_id, modifier_mask, value, auto_reset);
    }
}
