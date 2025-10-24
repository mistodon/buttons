//! Type aliases for input devices that work with `winit`.

use crate::winit::{
    self,
    event::{Event as WinitEvent, WindowEvent},
};
use crate::{Event, Keyboard, Mouse, Touchpad, prelude::*};

/// Alias for a type that represents a keyboard key code.
#[cfg(any(feature = "winit_0_29", feature = "winit_0_30"))]
pub type WinitKey = winit::keyboard::KeyCode;

/// Alias for a type that represents a keyboard key code.
#[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
pub type WinitKey = winit::event::VirtualKeyCode;

/// Alias for a type that represents the state of keyboard modifiers.
#[cfg(any(feature = "winit_0_29", feature = "winit_0_30"))]
pub type WinitMods = winit::event::Modifiers;

/// Alias for a type that represents the state of keyboard modifiers.
#[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
pub type WinitMods = winit::event::ModifiersState;

/// Alias for a type that represents a mouse button.
pub type WinitMouseButton = winit::event::MouseButton;

/// Alias for a `Keyboard` that can represent `winit` keyboard state.
pub type WinitKeyboard = Keyboard<WinitKey, WinitMods>;

/// Alias for a `Mouse` that can represent `winit` mouse state.
pub type WinitMouse = Mouse<WinitMouseButton, f64>;

/// Alias for a `Touchpad` that can represent `winit` touch state.
pub type WinitTouchpad = Touchpad<u64, f64>;

/// Create a new WinitKeyboard.
pub fn keyboard() -> WinitKeyboard {
    WinitKeyboard::new()
}

/// Create a new WinitMouse.
pub fn mouse() -> WinitMouse {
    WinitMouse::new()
}

/// Create a new WinitMouse.
pub fn touch() -> WinitTouchpad {
    WinitTouchpad::new()
}

// winit >= 0.29 event handlers
#[cfg(any(feature = "winit_0_29", feature = "winit_0_30"))]
impl<T> Event<WinitKeyboard> for WinitEvent<T> {
    fn handle(&self, keyboard: &mut WinitKeyboard) {
        if let WinitEvent::WindowEvent { event, .. } = self {
            use winit::event::ElementState;

            match event {
                WindowEvent::KeyboardInput { event, .. } => {
                    let winit::event::KeyEvent {
                        state,
                        physical_key,
                        text,
                        ..
                    } = event;
                    if let winit::keyboard::PhysicalKey::Code(code) = physical_key {
                        match state {
                            ElementState::Pressed => keyboard.press(*code),
                            ElementState::Released => keyboard.release(*code),
                        };
                    }

                    if let Some(text) = text {
                        if !text.is_empty() {
                            keyboard.receive_text(text);
                        }
                    }
                }
                WindowEvent::ModifiersChanged(state) => {
                    keyboard.set_modifiers(*state);
                }
                _ => (),
            }
        }
    }
}

#[cfg(any(feature = "winit_0_29", feature = "winit_0_30"))]
impl<T> Event<WinitMouse> for WinitEvent<T> {
    fn handle(&self, mouse: &mut WinitMouse) {
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

#[cfg(any(feature = "winit_0_29", feature = "winit_0_30"))]
impl<T> Event<Touchpad<u64, f64>> for WinitEvent<T> {
    fn handle(&self, touchpad: &mut Touchpad<u64, f64>) {
        if let WinitEvent::WindowEvent { event, .. } = self {
            {
                use winit::event::TouchPhase;

                if let WindowEvent::Touch(touch) = event {
                    let pos = [touch.location.x, touch.location.y];
                    let phase = match touch.phase {
                        TouchPhase::Started => crate::touch::TouchPhase::Start,
                        TouchPhase::Ended => crate::touch::TouchPhase::End,
                        TouchPhase::Moved => crate::touch::TouchPhase::Move,
                        TouchPhase::Cancelled => crate::touch::TouchPhase::Cancel,
                    };
                    touchpad.touch_event(touch.id, pos, phase);
                }
            }
        }
    }
}

// winit < 0.29 event handlers
#[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
impl<T> Event<WinitKeyboard> for WinitEvent<'_, T> {
    fn handle(&self, keyboard: &mut WinitKeyboard) {
        if let WinitEvent::WindowEvent { event, .. } = self {
            use winit::event::ElementState;

            match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    let winit::event::KeyboardInput {
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

                WindowEvent::ReceivedCharacter(ch) => {
                    keyboard.receive_char(*ch);
                }

                #[cfg(not(feature = "winit_0_21"))]
                WindowEvent::ModifiersChanged(state) => {
                    keyboard.set_modifiers(*state);
                }
                _ => (),
            }
        }
    }
}

#[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
impl<T> Event<WinitMouse> for WinitEvent<'_, T> {
    fn handle(&self, mouse: &mut WinitMouse) {
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

#[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
impl<T> Event<Touchpad<u64, f64>> for WinitEvent<'_, T> {
    fn handle(&self, touchpad: &mut Touchpad<u64, f64>) {
        if let WinitEvent::WindowEvent { event, .. } = self {
            {
                use winit::event::TouchPhase;

                if let WindowEvent::Touch(touch) = event {
                    let pos = [touch.location.x, touch.location.y];
                    let phase = match touch.phase {
                        TouchPhase::Started => crate::touch::TouchPhase::Start,
                        TouchPhase::Ended => crate::touch::TouchPhase::End,
                        TouchPhase::Moved => crate::touch::TouchPhase::Move,
                        TouchPhase::Cancelled => crate::touch::TouchPhase::Cancel,
                    };
                    touchpad.touch_event(touch.id, pos, phase);
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(deprecated)]
#[allow(invalid_value)]
mod tests {
    use super::*;
    use crate::touch::TouchPhase;
    use winit::event::ElementState;
    use winit::event::TouchPhase as TP;

    #[cfg(any(feature = "winit_0_29", feature = "winit_0_30"))]
    const TEST_KEY: WinitKey = WinitKey::KeyH;

    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
    const TEST_KEY: WinitKey = WinitKey::H;

    #[test]
    fn create_devices() {
        let mut keyboard = keyboard();
        let mut mouse = mouse();
        let mut touchpad = touch();
        keyboard.press(TEST_KEY);
        keyboard.set_modifiers(WinitMods::default());
        mouse.press(WinitMouseButton::Left);
        mouse.move_to([0., 0.]);
        touchpad.touch_event(0_u64, [100., 100.], TouchPhase::Start);
    }

    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
    fn make_modifier_event(ctrl: bool) -> WinitEvent<'static, ()> {
        let state = if ctrl {
            WinitMods::CTRL
        } else {
            WinitMods::default()
        };
        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::ModifiersChanged(state),
            }
        }
    }

    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
    fn make_keyboard_event(pressed: bool, key: WinitKey) -> WinitEvent<'static, ()> {
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

    #[cfg(any(feature = "winit_0_29", feature = "winit_0_30"))]
    fn make_mouse_button_event(pressed: bool, button: WinitMouseButton) -> WinitEvent<()> {
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

                    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
                    modifiers: WinitMods::default(),
                },
            }
        }
    }

    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
    fn make_mouse_button_event(pressed: bool, button: WinitMouseButton) -> WinitEvent<'static, ()> {
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

                    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
                    modifiers: WinitMods::default(),
                },
            }
        }
    }

    #[cfg(any(feature = "winit_0_29", feature = "winit_0_30"))]
    fn make_touch_event(id: u64, pos: [f64; 2], phase: TP) -> WinitEvent<()> {
        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::Touch(winit::event::Touch {
                    device_id: ::std::mem::uninitialized(),
                    phase,
                    location: winit::dpi::PhysicalPosition::new(pos[0], pos[1]),
                    force: None,
                    id,
                }),
            }
        }
    }

    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
    fn make_touch_event(id: u64, pos: [f64; 2], phase: TP) -> WinitEvent<'static, ()> {
        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::Touch(winit::event::Touch {
                    device_id: ::std::mem::uninitialized(),
                    phase,
                    location: winit::dpi::PhysicalPosition::new(pos[0], pos[1]),
                    force: None,
                    id,
                }),
            }
        }
    }

    #[cfg(any(feature = "winit_0_29", feature = "winit_0_30"))]
    fn make_cursor_event(position: [f64; 2]) -> WinitEvent<()> {
        use winit::dpi::PhysicalPosition;

        let [x, y] = position;
        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::CursorMoved {
                    device_id: ::std::mem::uninitialized(),
                    position: PhysicalPosition { x, y },
                },
            }
        }
    }

    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
    fn make_cursor_event(position: [f64; 2]) -> WinitEvent<'static, ()> {
        use winit::dpi::PhysicalPosition;

        let [x, y] = position;
        unsafe {
            WinitEvent::WindowEvent {
                window_id: ::std::mem::uninitialized(),
                event: WindowEvent::CursorMoved {
                    device_id: ::std::mem::uninitialized(),
                    position: PhysicalPosition { x, y },
                    modifiers: WinitMods::default(),
                },
            }
        }
    }

    #[test]
    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
    fn press_via_event() {
        let event = make_keyboard_event(true, TEST_KEY);
        let mut keyboard = keyboard();
        keyboard.handle_event(&event);

        assert!(keyboard.pressed(&TEST_KEY));
    }

    #[test]
    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
    fn release_via_event() {
        let event = make_keyboard_event(false, TEST_KEY);
        let mut keyboard = keyboard();
        keyboard.handle_event(&event);

        assert!(keyboard.released(&TEST_KEY));
    }

    #[test]
    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
    fn modifiers_via_event() {
        let event = make_modifier_event(true);
        let mut keyboard = keyboard();
        keyboard.handle_event(&event);

        let modifiers = WinitMods::CTRL;
        assert_eq!(keyboard.modifiers(), Some(&modifiers));
    }

    #[test]
    #[cfg(not(any(feature = "winit_0_29", feature = "winit_0_30")))]
    fn modifiers_off_via_event() {
        let event = make_modifier_event(false);
        let mut keyboard = keyboard();
        keyboard.handle_event(&event);

        let modifiers = WinitMods::default();
        assert_eq!(keyboard.modifiers(), Some(&modifiers));
    }

    #[test]
    fn mouse_button_press_via_event() {
        let event = make_mouse_button_event(true, WinitMouseButton::Right);
        let mut mouse = mouse();
        mouse.handle_event(&event);

        assert!(mouse.pressed(&WinitMouseButton::Right));
    }

    #[test]
    fn mouse_button_release_via_event() {
        let event = make_mouse_button_event(false, WinitMouseButton::Right);
        let mut mouse = mouse();
        mouse.handle_event(&event);

        assert!(mouse.released(&WinitMouseButton::Right));
    }

    #[test]
    fn mouse_move_via_event() {
        let event = make_cursor_event([1., 1.]);
        let mut mouse = mouse();
        mouse.handle_event(&event);

        assert_eq!(mouse.position(), [1., 1.]);
    }

    #[test]
    fn touch_via_event() {
        let mut touch = touch();
        let event = make_touch_event(0, [1., 1.], TP::Started);
        touch.handle_event(&event);

        assert_eq!(
            touch.first_touch(),
            Some(&crate::touch::Touch {
                id: 0,
                position: [1., 1.],
                tapped: true,
                released: false,
            })
        );

        touch.clear_taps();

        assert_eq!(
            touch.first_touch(),
            Some(&crate::touch::Touch {
                id: 0,
                position: [1., 1.],
                tapped: false,
                released: false,
            })
        );

        let event = make_touch_event(0, [10., 10.], TP::Ended);
        touch.handle_event(&event);

        assert_eq!(
            touch.first_touch(),
            Some(&crate::touch::Touch {
                id: 0,
                position: [10., 10.],
                tapped: false,
                released: true,
            })
        );

        touch.clear_taps();

        assert_eq!(touch.first_touch(), None);
    }
}
