use crate::input::{AxisId, ButtonId, GuestureHandler, GuestureResponse, State};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ButtonMap {
    ScanCode(winit::ScanCode),
    VirtualKey(winit::VirtualKeyCode),
    //GamePad()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AxisMap {
    MouseAxis(u32),
    MouseAxisWithDevice(winit::DeviceId, u32),
    Gamepad(gilrs::ev::Axis),
    GamepadWithDevice(gilrs::GamepadId, gilrs::ev::Axis),
}

pub struct Mapping {
    axis_mapping: HashMap<AxisMap, (AxisId, f32)>,
    key_mapping: HashMap<ButtonMap, ButtonId>,
}

impl Mapping {
    pub fn new() -> Mapping {
        Mapping {
            axis_mapping: HashMap::new(),
            key_mapping: HashMap::new(),
        }
    }

    pub fn new_debug() -> Mapping {
        let mut mapping = Mapping::new();
        mapping.add_axis_mapping(AxisMap::MouseAxis(0), AxisId::new(0), 0.1);
        mapping.add_axis_mapping(AxisMap::MouseAxis(1), AxisId::new(1), 0.1);
        mapping
    }

    pub fn add_axis_mapping(&mut self, from: AxisMap, to: AxisId, sensitivity: f32) {
        self.axis_mapping.insert(from, (to, sensitivity));
    }

    pub fn add_key_mapping(&mut self, from: ButtonMap, to: ButtonId) {
        self.key_mapping.insert(from, to);
    }

    pub fn map_winit_joystic(&self, device_id: &winit::DeviceId, axis: u32) -> Option<(AxisId, f32)> {
        let mapping = &self.axis_mapping;

        if let Some(axis) = mapping.get(&AxisMap::MouseAxisWithDevice(device_id.to_owned(), axis)) {
            return Some(axis.to_owned());
        }

        if let Some(axis) = mapping.get(&AxisMap::MouseAxis(axis)) {
            return Some(axis.to_owned());
        }

        None
    }

    pub fn map_winit_button(&self, key_input: &winit::KeyboardInput) -> Option<ButtonId> {
        if let Some(btn) = self.key_mapping.get(&ButtonMap::ScanCode(key_input.scancode)) {
            return Some(btn.to_owned());
        }

        if let Some(vk) = key_input.virtual_keycode {
            if let Some(btn) = self.key_mapping.get(&ButtonMap::VirtualKey(vk)) {
                return Some(btn.to_owned());
            }
        }

        return None;
    }

    pub fn map_gil_joystic(&self, device_id: &gilrs::GamepadId, axis: &gilrs::ev::Axis) -> Option<(AxisId, f32)> {
        let mapping = &self.axis_mapping;

        if let Some(axis) = mapping.get(&AxisMap::GamepadWithDevice(device_id.to_owned(), axis.to_owned())) {
            return Some(axis.to_owned());
        }

        if let Some(axis) = mapping.get(&AxisMap::Gamepad(axis.to_owned())) {
            return Some(axis.to_owned());
        }

        None
    }
}
