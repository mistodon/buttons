use Event;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub logo: bool,
}

#[derive(Debug, Clone)]
pub struct Keyboard<Key, Mods>
where
    Key: Copy + PartialEq,
    Mods: Copy + Default,
{
    modifiers: Mods,
    keys_down: Vec<Key>,
    keys_pressed: Vec<Key>,
    keys_released: Vec<Key>,
}

impl<Key, Mods> Default for Keyboard<Key, Mods>
where
    Key: Copy + PartialEq,
    Mods: Copy + Default,
{
    fn default() -> Self {
        Keyboard {
            modifiers: Default::default(),
            keys_down: Vec::with_capacity(4),
            keys_pressed: Vec::with_capacity(4),
            keys_released: Vec::with_capacity(4),
        }
    }
}

impl<Key, Mods> Keyboard<Key, Mods>
where
    Key: Copy + PartialEq,
    Mods: Copy + Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn modifiers(&self) -> Mods {
        self.modifiers
    }

    pub fn begin_frame_input(&mut self) -> KeyboardInput<Key, Mods> {
        self.keys_pressed.clear();
        self.keys_released.clear();
        KeyboardInput { keyboard: self }
    }

    pub fn down(&self, key: Key) -> bool {
        self.keys_down.iter().any(|&k| k == key)
    }

    pub fn pressed(&self, key: Key) -> bool {
        self.keys_pressed.iter().any(|&k| k == key)
    }

    pub fn released(&self, key: Key) -> bool {
        self.keys_released.iter().any(|&k| k == key)
    }
}

pub struct KeyboardInput<'a, Key, Mods>
where
    Key: Copy + PartialEq + 'a,
    Mods: Copy + Default + 'a,
{
    keyboard: &'a mut Keyboard<Key, Mods>,
}

impl<'a, Key, Mods> KeyboardInput<'a, Key, Mods>
where
    Key: Copy + PartialEq,
    Mods: Copy + Default,
{
    pub fn press(&mut self, key: Key) -> &mut Self {
        if !self.keyboard.down(key) {
            self.keyboard.keys_down.push(key);
        }
        if !self.keyboard.pressed(key) {
            self.keyboard.keys_pressed.push(key);
        }
        self
    }

    pub fn release(&mut self, key: Key) -> &mut Self {
        self.keyboard.keys_down.retain(|&k| k != key);
        if !self.keyboard.released(key) {
            self.keyboard.keys_released.push(key);
        }
        self
    }

    pub fn set_modifiers(&mut self, modifiers: Mods) -> &mut Self {
        self.keyboard.modifiers = modifiers;
        self
    }

    pub fn handle_event<E: Event<Self>>(&mut self, event: &E) -> &mut Self {
        event.handle(self);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_not_pressed_or_released_by_default() {
        let keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        assert!(!keyboard.down(10));
        assert!(!keyboard.pressed(10));
        assert!(!keyboard.released(10));
    }

    #[test]
    fn key_down_when_pressed() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        {
            keyboard.begin_frame_input().press(10);
        }
        assert!(keyboard.down(10));
    }

    #[test]
    fn key_not_down_when_released() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        {
            keyboard.begin_frame_input().press(10).release(10);
        }
        assert!(!keyboard.down(10));
    }

    #[test]
    fn key_pressed_after_pressing() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        {
            keyboard.begin_frame_input().press(10);
        }
        assert!(keyboard.pressed(10));
    }

    #[test]
    fn key_released_after_releasing() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        {
            keyboard.begin_frame_input().release(10);
        }
        assert!(keyboard.released(10));
    }

    #[test]
    fn key_can_be_pressed_and_released_on_same_frame() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        {
            keyboard.begin_frame_input().press(10).release(10);
        }
        assert!(keyboard.pressed(10));
        assert!(keyboard.released(10));
    }

    #[test]
    fn key_pressed_resets_at_start_of_frame() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
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
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
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
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
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
        let keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        assert_eq!(keyboard.modifiers(), Modifiers::default());
    }

    #[test]
    fn can_set_modifiers() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
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
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
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
