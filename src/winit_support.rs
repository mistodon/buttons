use winit::{Event as WinitEvent, ModifiersState, MouseButton, VirtualKeyCode, WindowEvent};

use crate::keyboard::{Keyboard, KeyboardInput};
use crate::mouse::{Mouse, MouseInput};
use crate::Event;

/// Alias for a `Keyboard` that can represent `winit` keyboard state.
pub type WinitKeyboard = Keyboard<VirtualKeyCode, ModifiersState>;

/// Alias for a `Mouse` that can represent `winit` mouse state.
pub type WinitMouse = Mouse<MouseButton, f64>;

/// Create a new WinitKeyboard.
pub fn keyboard() -> WinitKeyboard {
    Keyboard::new()
}

/// Create a new WinitMouse.
pub fn mouse() -> WinitMouse {
    Mouse::new()
}

impl<'a> Event<KeyboardInput<'a, VirtualKeyCode, ModifiersState>> for WinitEvent {
    fn handle(&self, keyboard: &mut KeyboardInput<VirtualKeyCode, ModifiersState>) {
        if let WinitEvent::WindowEvent { event, .. } = self {
            event.handle(keyboard);
        }
    }
}

impl<'a> Event<KeyboardInput<'a, VirtualKeyCode, ModifiersState>> for WindowEvent {
    fn handle(&self, keyboard: &mut KeyboardInput<VirtualKeyCode, ModifiersState>) {
        use winit::{ElementState, KeyboardInput};

        if let WindowEvent::KeyboardInput { input, .. } = self {
            let KeyboardInput {
                state,
                virtual_keycode,
                modifiers,
                ..
            } = input;
            keyboard.set_modifiers(*modifiers);
            if let Some(vkc) = virtual_keycode {
                match state {
                    ElementState::Pressed => keyboard.press(*vkc),
                    ElementState::Released => keyboard.release(*vkc),
                };
            }
        }
    }
}

impl<'a> Event<MouseInput<'a, MouseButton, f64>> for WinitEvent {
    fn handle(&self, mouse: &mut MouseInput<MouseButton, f64>) {
        if let WinitEvent::WindowEvent { event, .. } = self {
            event.handle(mouse);
        }
    }
}

impl<'a> Event<MouseInput<'a, MouseButton, f64>> for WindowEvent {
    fn handle(&self, mouse: &mut MouseInput<MouseButton, f64>) {
        use winit::ElementState;

        match self {
            WindowEvent::MouseInput { state, button, .. } => {
                match state {
                    ElementState::Pressed => mouse.press(*button),
                    ElementState::Released => mouse.release(*button),
                };
            }
            WindowEvent::CursorMoved { position, .. } => {
                mouse.move_to([position.x, position.y]);
            }
            _ => (),
        }
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use winit::{self, ElementState};

    #[test]
    fn create_mouse_and_keyboard() {
        let mut keyboard = keyboard();
        let mut mouse = mouse();
        {
            let keyboard = &mut keyboard.begin_frame_input();
            let mouse = &mut mouse.begin_frame_input();
            keyboard.press(VirtualKeyCode::H);
            keyboard.set_modifiers(ModifiersState::default());
            mouse.press(MouseButton::Left);
            mouse.move_to([0.0, 0.0]);
        }
    }

    fn make_keyboard_event(pressed: bool, key: VirtualKeyCode, ctrl: bool) -> WinitEvent {
        let state = match pressed {
            true => ElementState::Pressed,
            false => ElementState::Released,
        };

        let modifiers = ModifiersState {
            ctrl,
            ..Default::default()
        };

        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::KeyboardInput {
                    device_id: ::std::mem::uninitialized(),
                    input: winit::KeyboardInput {
                        scancode: 0,
                        state,
                        virtual_keycode: Some(key),
                        modifiers,
                    },
                },
            }
        }
    }

    fn make_mouse_button_event(pressed: bool, button: MouseButton) -> WinitEvent {
        let state = match pressed {
            true => ElementState::Pressed,
            false => ElementState::Released,
        };

        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::MouseInput {
                    device_id: ::std::mem::uninitialized(),
                    state,
                    button,
                    modifiers: ModifiersState::default(),
                },
            }
        }
    }

    fn make_cursor_event(position: [f64; 2]) -> WinitEvent {
        use winit::dpi::LogicalPosition;

        let [x, y] = position;
        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::CursorMoved {
                    device_id: ::std::mem::uninitialized(),
                    position: LogicalPosition { x, y },
                    modifiers: ModifiersState::default(),
                },
            }
        }
    }

    #[test]
    fn press_via_event() {
        let event = make_keyboard_event(true, VirtualKeyCode::H, false);
        let mut keyboard = keyboard();
        {
            keyboard.begin_frame_input().handle_event(&event);
        }
        assert!(keyboard.pressed(VirtualKeyCode::H));
    }

    #[test]
    fn release_via_event() {
        let event = make_keyboard_event(false, VirtualKeyCode::H, false);
        let mut keyboard = keyboard();
        {
            keyboard.begin_frame_input().handle_event(&event);
        }
        assert!(keyboard.released(VirtualKeyCode::H));
    }

    #[test]
    fn modifiers_via_event() {
        let event = make_keyboard_event(false, VirtualKeyCode::H, true);
        let mut keyboard = keyboard();
        {
            keyboard.begin_frame_input().handle_event(&event);
        }
        let modifiers = ModifiersState {
            ctrl: true,
            ..Default::default()
        };
        assert_eq!(keyboard.modifiers(), modifiers);
    }

    #[test]
    fn mouse_button_press_via_event() {
        let event = make_mouse_button_event(true, MouseButton::Right);
        let mut mouse = mouse();
        {
            mouse.begin_frame_input().handle_event(&event);
        }
        assert!(mouse.pressed(MouseButton::Right));
    }

    #[test]
    fn mouse_button_release_via_event() {
        let event = make_mouse_button_event(false, MouseButton::Right);
        let mut mouse = mouse();
        {
            mouse.begin_frame_input().handle_event(&event);
        }
        assert!(mouse.released(MouseButton::Right));
    }

    #[test]
    fn mouse_move_via_event() {
        let event = make_cursor_event([1.0, 1.0]);
        let mut mouse = mouse();
        {
            mouse.begin_frame_input().handle_event(&event);
        }
        assert_eq!(mouse.position(), [1.0, 1.0]);
    }
}
