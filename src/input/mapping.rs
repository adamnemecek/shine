use crate::input::AxisId;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum InputMapping {
    ScanCodeKey(winit::ScanCode),
    VirtualKey(winit::VirtualKeyCode),
    MouseAxis(u32),
    MouseAxisWithDevice(winit::DeviceId, u32),
    Gamepad(gilrs::ev::Axis),
    GamepadWithDevice(gilrs::GamepadId, gilrs::ev::Axis),
}

pub struct Mapping {
    axis_mapping: HashMap<InputMapping, (AxisId, f32)>,
    modifier_mapping: HashMap<InputMapping, ModifierId>,
}

impl Mapping {
    pub fn new() -> Mapping {
        Mapping {
            axis_mapping: HashMap::new(),
            modifier_mapping: HashMap::new(),
        }
    }

    pub fn add_axis_mapping(&mut self, from: InputMapping, to: AxisId, sensitivity: f32) {
        self.axis_mapping.insert(from, (to, sensitivity));
    }

    pub fn add_modifier_mapping(&mut self, from: InputMapping, to: ModifierId) {
        assert!(to < MAX_MODIFIER_COUNT);
        self.modifier_mapping.insert(from, to);
    }

    pub fn map_winit_axis_to_modifier(&self, device_id: &winit::DeviceId, axis: u32) -> Option<ModifierId> {
        let mapping = &self.modifier_mapping;

        if let Some(res) = mapping.get(&InputMapping::MouseAxisWithDevice(device_id.to_owned(), axis)) {
            return Some(res.to_owned());
        }

        if let Some(res) = mapping.get(&InputMapping::MouseAxis(axis)) {
            return Some(res.to_owned());
        }

        None
    }

    pub fn map_winit_key_to_modifier(&self, key_input: &winit::KeyboardInput) -> Option<ModifierId> {
        let mapping = &self.modifier_mapping;

        if let Some(res) = mapping.get(&InputMapping::ScanCodeKey(key_input.scancode)) {
            return Some(res.to_owned());
        }

        if let Some(vk) = key_input.virtual_keycode {
            if let Some(res) = mapping.get(&InputMapping::VirtualKey(vk)) {
                return Some(res.to_owned());
            }
        }

        return None;
    }

    pub fn map_winit_axis_to_axis(&self, device_id: &winit::DeviceId, axis: u32) -> Option<(AxisId, f32)> {
        let mapping = &self.axis_mapping;

        if let Some(res) = mapping.get(&InputMapping::MouseAxisWithDevice(device_id.to_owned(), axis)) {
            return Some(res.to_owned());
        }

        if let Some(res) = mapping.get(&InputMapping::MouseAxis(axis)) {
            return Some(res.to_owned());
        }

        None
    }

    pub fn map_winit_key_to_axis(&self, key_input: &winit::KeyboardInput) -> Option<(AxisId, f32)> {
        let mapping = &self.axis_mapping;

        if let Some(res) = mapping.get(&InputMapping::ScanCodeKey(key_input.scancode)) {
            return Some(res.to_owned());
        }

        if let Some(vk) = key_input.virtual_keycode {
            if let Some(res) = mapping.get(&InputMapping::VirtualKey(vk)) {
                return Some(res.to_owned());
            }
        }

        return None;
    }

    pub fn map_gil_axis_to_modifier(&self, device_id: &gilrs::GamepadId, axis: &gilrs::ev::Axis) -> Option<ModifierId> {
        let mapping = &self.modifier_mapping;

        if let Some(res) = mapping.get(&InputMapping::GamepadWithDevice(device_id.to_owned(), axis.to_owned())) {
            return Some(res.to_owned());
        }

        if let Some(res) = mapping.get(&InputMapping::Gamepad(axis.to_owned())) {
            return Some(res.to_owned());
        }

        None
    }

    pub fn map_gil_axis_to_axis(&self, device_id: &gilrs::GamepadId, axis: &gilrs::ev::Axis) -> Option<(AxisId, f32)> {
        let mapping = &self.axis_mapping;

        if let Some(res) = mapping.get(&InputMapping::GamepadWithDevice(device_id.to_owned(), axis.to_owned())) {
            return Some(res.to_owned());
        }

        if let Some(res) = mapping.get(&InputMapping::Gamepad(axis.to_owned())) {
            return Some(res.to_owned());
        }

        None
    }
}
