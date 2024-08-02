use winit::event::{Event as WinitEvent, Modifiers, MouseButton, WindowEvent};
use winit::keyboard::Key;

use crate::{Event, Keyboard, Mouse, Touchpad};

/// Alias for a `Keyboard` that can represent `winit` keyboard state.
pub type WinitKeyboard = Keyboard<Key, Modifiers>;

/// Alias for a `Mouse` that can represent `winit` mouse state.
pub type WinitMouse = Mouse<MouseButton, f64>;

/// Alias for a `Touchpad` that can represent `winit` touch state.
pub type WinitTouchpad = Touchpad<u64, f64>;

/// Create a new WinitKeyboard.
pub fn keyboard() -> WinitKeyboard {
    Keyboard::new()
}

/// Create a new WinitMouse.
pub fn mouse() -> WinitMouse {
    Mouse::new()
}

/// Create a new WinitMouse.
pub fn touch() -> WinitTouchpad {
    Touchpad::new()
}

impl<T> Event<WinitKeyboard> for WinitEvent<T> {
    fn handle(&self, keyboard: &mut WinitKeyboard) {
        if let WinitEvent::WindowEvent { event, .. } = self {
            use winit::event::{ElementState};

            match event {
                WindowEvent::KeyboardInput { event, .. } => {
                    // TODO: Pretty inelegant
                    if let Some(text) = &event.text {
                        for ch in text.chars() {
                            keyboard.receive_char(ch);
                        }
                    }

                    match event.state {
                        ElementState::Pressed => keyboard.press(event.logical_key.clone()),
                        ElementState::Released => keyboard.release(event.logical_key.clone()),
                    };
                }
                WindowEvent::ModifiersChanged(state) => {
                    keyboard.set_modifiers(*state);
                }
                _ => (),
            }
        }
    }
}

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

// TODO: You have to test this, friend
// #[cfg(test)]
// #[allow(deprecated)]
// #[allow(invalid_value)]
// mod tests {
//     use super::*;
//     use crate::touch::TouchPhase;
//     use winit::event::ElementState;
//     use winit::event::TouchPhase as TP;

//     #[test]
//     fn create_devices() {
//         let mut keyboard = keyboard();
//         let mut mouse = mouse();
//         let mut touchpad = touch();
//         keyboard.press(NamedKey::H);
//         keyboard.set_modifiers(ModifiersState::default());
//         mouse.press(MouseButton::Left);
//         mouse.move_to([0., 0.]);
//         touchpad.touch_event(0_u64, [100., 100.], TouchPhase::Start);
//     }

//     fn make_modifier_event(ctrl: bool) -> WinitEvent<'static, ()> {
//         let state = if ctrl {
//             ModifiersState::CTRL
//         } else {
//             ModifiersState::default()
//         };
//         unsafe {
//             WinitEvent::WindowEvent {
//                 window_id: ::std::mem::uninitialized(),
//                 event: WindowEvent::ModifiersChanged(state),
//             }
//         }
//     }

//     fn make_keyboard_event(pressed: bool, key: NamedKey) -> WinitEvent<'static, ()> {
//         let state = match pressed {
//             true => ElementState::Pressed,
//             false => ElementState::Released,
//         };

//         unsafe {
//             WinitEvent::WindowEvent {
//                 window_id: ::std::mem::uninitialized(),
//                 event: WindowEvent::KeyboardInput {
//                     device_id: ::std::mem::uninitialized(),
//                     input: winit::event::KeyboardInput {
//                         scancode: 0,
//                         state,
//                         virtual_keycode: Some(key),
//                         modifiers: Default::default(),
//                     },
//                     is_synthetic: false,
//                 },
//             }
//         }
//     }

//     fn make_mouse_button_event(pressed: bool, button: MouseButton) -> WinitEvent<'static, ()> {
//         let state = match pressed {
//             true => ElementState::Pressed,
//             false => ElementState::Released,
//         };

//         unsafe {
//             WinitEvent::WindowEvent {
//                 window_id: ::std::mem::uninitialized(),
//                 event: WindowEvent::MouseInput {
//                     device_id: ::std::mem::uninitialized(),
//                     state,
//                     button,
//                 },
//             }
//         }
//     }

//     fn make_touch_event(id: u64, pos: [f64; 2], phase: TP) -> WinitEvent<'static, ()> {
//         unsafe {
//             WinitEvent::WindowEvent {
//                 window_id: ::std::mem::uninitialized(),
//                 event: WindowEvent::Touch(winit::event::Touch {
//                     device_id: ::std::mem::uninitialized(),
//                     phase,
//                     location: winit::dpi::PhysicalPosition::new(pos[0], pos[1]),
//                     force: None,
//                     id,
//                 }),
//             }
//         }
//     }

//     fn make_cursor_event(position: [f64; 2]) -> WinitEvent<'static, ()> {
//         use winit::dpi::PhysicalPosition;

//         let [x, y] = position;
//         unsafe {
//             WinitEvent::WindowEvent {
//                 window_id: ::std::mem::uninitialized(),
//                 event: WindowEvent::CursorMoved {
//                     device_id: ::std::mem::uninitialized(),
//                     position: PhysicalPosition { x, y },
//                 },
//             }
//         }
//     }

//     #[test]
//     fn press_via_event() {
//         let event = make_keyboard_event(true, NamedKey::H);
//         let mut keyboard = keyboard();
//         keyboard.handle_event(&event);

//         assert!(keyboard.pressed(NamedKey::H));
//     }

//     #[test]
//     fn release_via_event() {
//         let event = make_keyboard_event(false, NamedKey::H);
//         let mut keyboard = keyboard();
//         keyboard.handle_event(&event);

//         assert!(keyboard.released(NamedKey::H));
//     }

//     #[test]
//     fn modifiers_via_event() {
//         let event = make_modifier_event(true);
//         let mut keyboard = keyboard();
//         keyboard.handle_event(&event);

//         let modifiers = ModifiersState::CTRL;
//         assert_eq!(keyboard.modifiers(), modifiers);
//     }

//     #[test]
//     fn modifiers_off_via_event() {
//         let event = make_modifier_event(false);
//         let mut keyboard = keyboard();
//         keyboard.handle_event(&event);

//         let modifiers = ModifiersState::default();
//         assert_eq!(keyboard.modifiers(), modifiers);
//     }

//     #[test]
//     fn mouse_button_press_via_event() {
//         let event = make_mouse_button_event(true, MouseButton::Right);
//         let mut mouse = mouse();
//         mouse.handle_event(&event);

//         assert!(mouse.pressed(MouseButton::Right));
//     }

//     #[test]
//     fn mouse_button_release_via_event() {
//         let event = make_mouse_button_event(false, MouseButton::Right);
//         let mut mouse = mouse();
//         mouse.handle_event(&event);

//         assert!(mouse.released(MouseButton::Right));
//     }

//     #[test]
//     fn mouse_move_via_event() {
//         let event = make_cursor_event([1., 1.]);
//         let mut mouse = mouse();
//         mouse.handle_event(&event);

//         assert_eq!(mouse.position(), [1., 1.]);
//     }

//     #[test]
//     fn touch_via_event() {
//         let mut touch = touch();
//         let event = make_touch_event(0, [1., 1.], TP::Started);
//         touch.handle_event(&event);

//         assert_eq!(
//             touch.first_touch(),
//             Some(&crate::touch::Touch {
//                 id: 0,
//                 position: [1., 1.],
//                 tapped: true,
//                 released: false,
//             })
//         );

//         touch.clear_taps();

//         assert_eq!(
//             touch.first_touch(),
//             Some(&crate::touch::Touch {
//                 id: 0,
//                 position: [1., 1.],
//                 tapped: false,
//                 released: false,
//             })
//         );

//         let event = make_touch_event(0, [10., 10.], TP::Ended);
//         touch.handle_event(&event);

//         assert_eq!(
//             touch.first_touch(),
//             Some(&crate::touch::Touch {
//                 id: 0,
//                 position: [10., 10.],
//                 tapped: false,
//                 released: true,
//             })
//         );

//         touch.clear_taps();

//         assert_eq!(touch.first_touch(), None);
//     }
// }
