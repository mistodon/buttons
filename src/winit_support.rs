use winit::{MouseButton, VirtualKeyCode};

use keyboard::KeyId;
use mouse::MouseButtonId;

impl KeyId for VirtualKeyCode {
    fn key_code(self) -> u8 {
        self as u8
    }

    fn from_key_code(_key_code: u8) -> Option<Self> {
        None
    }
}

impl MouseButtonId for MouseButton {
    fn button_id(self) -> usize {
        match self {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            MouseButton::Middle => 2,
            MouseButton::Other(n) => n as usize,
        }
    }

    fn from_button_id(button_id: usize) -> Option<Self> {
        match button_id {
            0 => Some(MouseButton::Left),
            1 => Some(MouseButton::Right),
            2 => Some(MouseButton::Middle),
            n if n < 8 => Some(MouseButton::Other(n as u8)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use keyboard::Keyboard;
    use mouse::Mouse;

    #[test]
    fn can_use_winit_keys() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().press(VirtualKeyCode::Escape);
        }
        assert!(keyboard.pressed(VirtualKeyCode::Escape));
    }

    #[test]
    fn can_use_winit_mouse() {
        let mut mouse = Mouse::new();
        {
            mouse.begin_frame_input().press(MouseButton::Left);
        }
        assert!(mouse.pressed(MouseButton::Left));
    }

    #[test]
    fn key_conversion_consistency() {
        for key_code in 0..255_u8 {
            let vkc = VirtualKeyCode::from_key_code(key_code);
            if let Some(vkc) = vkc {
                assert_eq!(key_code, vkc.key_code());
            }
        }
    }

    #[test]
    fn mouse_button_conversion_consistency() {
        for button_id in 0..8 {
            let button = MouseButton::from_button_id(button_id);
            if let Some(button) = button {
                assert_eq!(button_id, button.button_id());
            }
        }
    }
}
