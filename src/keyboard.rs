pub trait KeyId: Copy {
    fn key_code(self) -> u8;
    fn from_key_code(key_code: u8) -> Option<Self>
    where
        Self: Sized;
}

impl KeyId for u8 {
    fn key_code(self) -> u8 {
        self
    }

    fn from_key_code(key_code: u8) -> Option<Self> {
        Some(key_code)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub logo: bool,
}

#[derive(Clone)]
pub struct Keyboard {
    modifiers: Modifiers,
    keys_down: [bool; 256],
    keys_pressed: [bool; 256],
    keys_released: [bool; 256],
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            modifiers: Modifiers::default(),
            keys_down: [false; 256],
            keys_pressed: [false; 256],
            keys_released: [false; 256],
        }
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn modifiers(&self) -> Modifiers {
        self.modifiers
    }

    pub fn begin_frame_input(&mut self) -> KeyboardInput {
        self.keys_pressed = [false; 256];
        self.keys_released = [false; 256];
        KeyboardInput { keyboard: self }
    }

    pub fn down<Key: KeyId>(&self, key: Key) -> bool {
        self.keys_down[key.key_code() as usize]
    }

    pub fn pressed<Key: KeyId>(&self, key: Key) -> bool {
        self.keys_pressed[key.key_code() as usize]
    }

    pub fn released<Key: KeyId>(&self, key: Key) -> bool {
        self.keys_released[key.key_code() as usize]
    }
}

pub struct KeyboardInput<'a> {
    keyboard: &'a mut Keyboard,
}

impl<'a> KeyboardInput<'a> {
    pub fn press<Key: KeyId>(&mut self, key: Key) -> &mut Self {
        self.keyboard.keys_down[key.key_code() as usize] = true;
        self.keyboard.keys_pressed[key.key_code() as usize] = true;
        self
    }

    pub fn release<Key: KeyId>(&mut self, key: Key) -> &mut Self {
        self.keyboard.keys_down[key.key_code() as usize] = false;
        self.keyboard.keys_released[key.key_code() as usize] = true;
        self
    }

    pub fn set_modifiers<M: Into<Modifiers>>(&mut self, modifiers: M) -> &mut Self {
        self.keyboard.modifiers = modifiers.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_not_pressed_or_released_by_default() {
        let keyboard = Keyboard::new();
        assert!(!keyboard.down(10));
        assert!(!keyboard.pressed(10));
        assert!(!keyboard.released(10));
    }

    #[test]
    fn key_down_when_pressed() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().press(10);
        }
        assert!(keyboard.down(10));
    }

    #[test]
    fn key_not_down_when_released() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().press(10).release(10);
        }
        assert!(!keyboard.down(10));
    }

    #[test]
    fn key_pressed_after_pressing() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().press(10);
        }
        assert!(keyboard.pressed(10));
    }

    #[test]
    fn key_released_after_releasing() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().release(10);
        }
        assert!(keyboard.released(10));
    }

    #[test]
    fn key_can_be_pressed_and_released_on_same_frame() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().press(10).release(10);
        }
        assert!(keyboard.pressed(10));
        assert!(keyboard.released(10));
    }

    #[test]
    fn key_pressed_resets_at_start_of_frame() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().press(10);
        }
        {
            keyboard.begin_frame_input();
        }
        assert!(!keyboard.pressed(10));
    }

    #[test]
    fn key_released_resets_at_start_of_frame() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().release(10);
        }
        {
            keyboard.begin_frame_input();
        }
        assert!(!keyboard.pressed(10));
    }

    #[test]
    fn key_down_persists_across_frames() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().press(10);
        }
        {
            keyboard.begin_frame_input();
        }
        assert!(keyboard.down(10));
    }

    #[test]
    fn modifiers_empty_by_default() {
        let keyboard = Keyboard::new();
        assert_eq!(keyboard.modifiers(), Modifiers::default());
    }

    #[test]
    fn can_set_modifiers() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().set_modifiers(Modifiers {
                ctrl: true,
                alt: true,
                shift: true,
                logo: true,
            });
        }
        assert_eq!(
            keyboard.modifiers(),
            Modifiers {
                ctrl: true,
                alt: true,
                shift: true,
                logo: true,
            }
        )
    }

    #[test]
    fn modifiers_persisit_over_frames() {
        let mut keyboard = Keyboard::new();
        {
            keyboard.begin_frame_input().set_modifiers(Modifiers {
                ctrl: true,
                alt: true,
                shift: true,
                logo: true,
            });
        }
        {
            keyboard.begin_frame_input();
        }
        assert_eq!(
            keyboard.modifiers(),
            Modifiers {
                ctrl: true,
                alt: true,
                shift: true,
                logo: true,
            }
        )
    }
}
