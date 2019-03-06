use crate::input::{ButtonId, ModifierFilter, ModifierFilterMask, ModifierId};
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
    axis_mapping: HashMap<InputMapping, (ButtonId, ModifierFilterMask, f32)>,
    modifier_mapping: HashMap<InputMapping, ModifierId>,
}

impl Mapping {
    pub fn new() -> Mapping {
        Mapping {
            axis_mapping: HashMap::new(),
            modifier_mapping: HashMap::new(),
        }
    }

    pub fn add_modifier_mapping(&mut self, input_event: InputMapping, modifier_id: ModifierId) {
        self.modifier_mapping.insert(input_event, modifier_id);
    }

    pub fn add_axis_mapping(
        &mut self,
        input_event: InputMapping,
        input_modifiers: Option<&[(ModifierId, ModifierFilter)]>,
        axis_id: ButtonId,
        sensitivity: f32,
    ) {
        let mask = match input_modifiers {
            None => ModifierFilterMask::default(),
            Some(f) => ModifierFilterMask::from(f),
        };
        self.axis_mapping.insert(input_event, (axis_id, mask, sensitivity));
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

    pub fn map_winit_axis_to_axis(&self, device_id: &winit::DeviceId, axis: u32) -> Option<(ButtonId, ModifierFilterMask, f32)> {
        let mapping = &self.axis_mapping;

        if let Some(res) = mapping.get(&InputMapping::MouseAxisWithDevice(device_id.to_owned(), axis)) {
            return Some(res.to_owned());
        }

        if let Some(res) = mapping.get(&InputMapping::MouseAxis(axis)) {
            return Some(res.to_owned());
        }

        None
    }

    pub fn map_winit_key_to_axis(&self, key_input: &winit::KeyboardInput) -> Option<(ButtonId, ModifierFilterMask, f32)> {
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

    pub fn map_gil_axis_to_axis(
        &self,
        device_id: &gilrs::GamepadId,
        axis: &gilrs::ev::Axis,
    ) -> Option<(ButtonId, ModifierFilterMask, f32)> {
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
