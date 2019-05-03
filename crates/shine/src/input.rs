pub use shine_input::Manager as InputManager;
use shine_input::{guestures, InputMapping};

pub mod buttons {
    use shine_input::ButtonId;

    pub const MOVE_FORWARD_POS: ButtonId = ButtonId::new(0);
    pub const MOVE_FORWARD_NEG: ButtonId = ButtonId::new(1);
    pub const MOVE_FORWARD: ButtonId = ButtonId::new(2);

    pub const MOVE_SIDE_POS: ButtonId = ButtonId::new(3);
    pub const MOVE_SIDE_NEG: ButtonId = ButtonId::new(4);
    pub const MOVE_SIDE: ButtonId = ButtonId::new(5);

    pub const MOVE_UP_POS: ButtonId = ButtonId::new(6);
    pub const MOVE_UP_NEG: ButtonId = ButtonId::new(7);
    pub const MOVE_UP: ButtonId = ButtonId::new(8);

    pub const ROLL: ButtonId = ButtonId::new(9);
    pub const YAW: ButtonId = ButtonId::new(10);
    pub const PITCH: ButtonId = ButtonId::new(11);

}

pub mod modifiers {
    use shine_input::ModifierId;

    pub const LSHIFT: ModifierId = ModifierId::new(0);
    pub const RSHIFT: ModifierId = ModifierId::new(1);
    pub const LCONTROL: ModifierId = ModifierId::new(2);
    pub const RCONTROL: ModifierId = ModifierId::new(3);
    pub const LALT: ModifierId = ModifierId::new(4);
    pub const RALT: ModifierId = ModifierId::new(5);
}

pub fn create_input_manager() -> InputManager {
    use buttons::*;
    use modifiers::*;
    use winit::VirtualKeyCode;

    let mut input_manager = InputManager::new();

    input_manager.add_modifier_mapping(InputMapping::VirtualKey(VirtualKeyCode::LShift), LSHIFT);
    input_manager.add_modifier_mapping(InputMapping::VirtualKey(VirtualKeyCode::RShift), RSHIFT);
    input_manager.add_modifier_mapping(InputMapping::VirtualKey(VirtualKeyCode::LControl), LCONTROL);
    input_manager.add_modifier_mapping(InputMapping::VirtualKey(VirtualKeyCode::RControl), RCONTROL);
    input_manager.add_modifier_mapping(InputMapping::VirtualKey(VirtualKeyCode::LAlt), LALT);
    input_manager.add_modifier_mapping(InputMapping::VirtualKey(VirtualKeyCode::RAlt), RALT);

    input_manager.add_button_mapping(InputMapping::ScanCodeKey(17), None, MOVE_FORWARD_POS, 1.); // W
    input_manager.add_button_mapping(InputMapping::ScanCodeKey(31), None, MOVE_FORWARD_NEG, 1.); // S
    input_manager.add_button_mapping(InputMapping::ScanCodeKey(30), None, MOVE_SIDE_NEG, 1.); // A
    input_manager.add_button_mapping(InputMapping::ScanCodeKey(32), None, MOVE_SIDE_POS, 1.); // D
    input_manager.add_button_mapping(InputMapping::ScanCodeKey(19), None, MOVE_UP_POS, 1.); // R
    input_manager.add_button_mapping(InputMapping::ScanCodeKey(33), None, MOVE_UP_NEG, 1.); // F

    input_manager.add_button_mapping(InputMapping::MouseAxis(0), None, YAW, -0.1); // mouse x
    input_manager.add_button_mapping(InputMapping::MouseAxis(1), None, PITCH, -0.1); // mouse y

    let mut key_joystick = guestures::KeyboardJoystick::default();
    key_joystick.add_axis(MOVE_FORWARD_POS, MOVE_FORWARD_NEG, MOVE_FORWARD, None.into());
    key_joystick.add_axis(MOVE_SIDE_POS, MOVE_SIDE_NEG, MOVE_SIDE, None.into());
    key_joystick.add_axis(MOVE_UP_POS, MOVE_UP_NEG, MOVE_UP, None.into());

    input_manager.add_guesture("key_joystick", Box::new(key_joystick));

    input_manager
}
