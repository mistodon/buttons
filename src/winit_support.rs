use winit::event::{Event as WinitEvent, ModifiersState, MouseButton, VirtualKeyCode, WindowEvent};

use crate::keyboard::Keyboard;
use crate::mouse::Mouse;
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

impl<'a, 'b, T> Event<Keyboard<VirtualKeyCode, ModifiersState>> for WinitEvent<'b, T> {
    fn handle(&self, keyboard: &mut Keyboard<VirtualKeyCode, ModifiersState>) {
        if let WinitEvent::WindowEvent { event, .. } = self {
            use winit::event::{ElementState, KeyboardInput};

            match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    let KeyboardInput {
                        state,
                        virtual_keycode,
                        ..
                    } = input;
                    if let Some(vkc) = virtual_keycode {
                        match state {
                            ElementState::Pressed => keyboard.press(*vkc),
                            ElementState::Released => keyboard.release(*vkc),
                        };
                    }
                }
                // WindowEvent::ModifiersChanged(state) => {
                //     keyboard.set_modifiers(*state);
                // }
                _ => (),
            }
        }
    }
}

impl<'a, 'b, T> Event<Mouse<MouseButton, f64>> for WinitEvent<'b, T> {
    fn handle(&self, mouse: &mut Mouse<MouseButton, f64>) {
        if let WinitEvent::WindowEvent { event, .. } = self {
            {
                use winit::event::ElementState;

                match event {
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
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use winit::event::ElementState;

    #[test]
    fn create_mouse_and_keyboard() {
        let mut keyboard = keyboard();
        let mut mouse = mouse();
        keyboard.press(VirtualKeyCode::H);
        keyboard.set_modifiers(ModifiersState::default());
        mouse.press(MouseButton::Left);
        mouse.move_to([0., 0.]);
    }

    fn make_modifier_event(ctrl: bool) -> WinitEvent<'static, ()> {
        let state = if ctrl {
            ModifiersState::CTRL
        } else {
            ModifiersState::default()
        };
        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::ModifiersChanged(state),
            }
        }
    }

    fn make_keyboard_event(pressed: bool, key: VirtualKeyCode) -> WinitEvent<'static, ()> {
        let state = match pressed {
            true => ElementState::Pressed,
            false => ElementState::Released,
        };

        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::KeyboardInput {
                    device_id: ::std::mem::uninitialized(),
                    input: winit::event::KeyboardInput {
                        scancode: 0,
                        state,
                        virtual_keycode: Some(key),
                        modifiers: Default::default(),
                    },
                    is_synthetic: false,
                },
            }
        }
    }

    fn make_mouse_button_event(pressed: bool, button: MouseButton) -> WinitEvent<'static, ()> {
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

    fn make_cursor_event(position: [f64; 2]) -> WinitEvent<'static, ()> {
        use winit::dpi::PhysicalPosition;

        let [x, y] = position;
        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::CursorMoved {
                    device_id: ::std::mem::uninitialized(),
                    position: PhysicalPosition { x, y },
                    modifiers: ModifiersState::default(),
                },
            }
        }
    }

    #[test]
    fn press_via_event() {
        let event = make_keyboard_event(true, VirtualKeyCode::H);
        let mut keyboard = keyboard();
        keyboard.handle_event(&event);

        assert!(keyboard.pressed(VirtualKeyCode::H));
    }

    #[test]
    fn release_via_event() {
        let event = make_keyboard_event(false, VirtualKeyCode::H);
        let mut keyboard = keyboard();
        keyboard.handle_event(&event);

        assert!(keyboard.released(VirtualKeyCode::H));
    }

    #[test]
    fn modifiers_via_event() {
        let event = make_modifier_event(true);
        let mut keyboard = keyboard();
        keyboard.handle_event(&event);

        let modifiers = ModifiersState::CTRL;
        assert_eq!(keyboard.modifiers(), modifiers);
    }

    #[test]
    fn modifiers_off_via_event() {
        let event = make_modifier_event(false);
        let mut keyboard = keyboard();
        keyboard.handle_event(&event);

        let modifiers = ModifiersState::default();
        assert_eq!(keyboard.modifiers(), modifiers);
    }

    #[test]
    fn mouse_button_press_via_event() {
        let event = make_mouse_button_event(true, MouseButton::Right);
        let mut mouse = mouse();
        mouse.handle_event(&event);

        assert!(mouse.pressed(MouseButton::Right));
    }

    #[test]
    fn mouse_button_release_via_event() {
        let event = make_mouse_button_event(false, MouseButton::Right);
        let mut mouse = mouse();
        mouse.handle_event(&event);

        assert!(mouse.released(MouseButton::Right));
    }

    #[test]
    fn mouse_move_via_event() {
        let event = make_cursor_event([1., 1.]);
        let mut mouse = mouse();
        mouse.handle_event(&event);

        assert_eq!(mouse.position(), [1., 1.]);
    }
}
