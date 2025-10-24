use smallvec::SmallVec;
use smol_str::{SmolStr, SmolStrBuilder};

use crate::Event;

// TODO: All these traits should take values by reference :')
/// A trait for objects that can represent the state of a keyboard.
pub trait KeyboardInterface {
    /// A type representing a key on a keyboard.
    type Key;

    /// A type representing the current state of modifier keys.
    type Mods;

    /// Returns the current state of the modifier keys, if present.
    fn modifiers(&self) -> Option<&Self::Mods>;

    /// Returns `true` if the given key is currently held down.
    fn down(&self, key: &Self::Key) -> bool;

    /// Returns `true` if the given key was pressed this frame.
    fn pressed(&self, key: &Self::Key) -> bool;

    /// Returns `true` if the given key was released this frame.
    fn released(&self, key: &Self::Key) -> bool;

    /// Returns any text that has been entered.
    fn text(&self) -> &str;

    /// Clears the pressed state of held buttons. Should be called at end of frame.
    fn clear_presses(&mut self) -> &mut Self;

    /// Register that a key was pressed down.
    fn press(&mut self, key: Self::Key) -> &mut Self;

    /// Register that a key was released.
    fn release(&mut self, key: Self::Key) -> &mut Self;

    /// Register that the current state of the modifier keys has changed.
    fn set_modifiers(&mut self, modifiers: Self::Mods) -> &mut Self;

    /// Register that some text was input.
    fn receive_text<S: AsRef<str>>(&mut self, text: S) -> &mut Self;

    /// Register that a character of text was input.
    fn receive_char(&mut self, ch: char) -> &mut Self;

    /// Convenience method for handling events. The type of event, `E`, will
    /// vary depending on the windowing library being used.
    fn handle_event<E: Event<Self>>(&mut self, event: &E) -> &mut Self {
        event.handle(self);
        self
    }
}

/// The current state of the modifier keys. You can use this if the windowing
/// library you are using doesn't have an equivalent.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub logo: bool,
}

/// A structure representing the current state of a keyboard.
#[derive(Debug, Clone)]
pub struct Keyboard<Key, Mods>
where
    // TODO: We should be able to relax these:
    // - If we store some flag other than a `Key` in keys_pressed/keys_released we can remove Clone for key
    // - If we rethink the `modifiers` API, we might be able to do away with Clone (but we don't necessarily want to force all implementations to own a Mods and return a ref)
    // - If we make modifiers optional, we can ditch the Default
    Key: Clone + PartialEq,
{
    modifiers: Option<Mods>,
    keys_down: SmallVec<[Key; 8]>,
    keys_pressed: SmallVec<[Key; 8]>,
    keys_released: SmallVec<[Key; 8]>,
    text_buffer_builder: SmolStrBuilder,
    text_buffer: SmolStr,
}

impl<Key, Mods> Keyboard<Key, Mods>
where
    Key: Clone + PartialEq,
{
    pub fn new() -> Self {
        Keyboard {
            modifiers: Default::default(),
            keys_down: Default::default(),
            keys_pressed: Default::default(),
            keys_released: Default::default(),
            text_buffer_builder: Default::default(),
            text_buffer: Default::default(),
        }
    }
}

impl<Key, Mods> Default for Keyboard<Key, Mods>
where
    Key: Clone + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, M> KeyboardInterface for Keyboard<K, M>
where
    K: Clone + PartialEq,
{
    type Key = K;
    type Mods = M;

    fn modifiers(&self) -> Option<&Self::Mods> {
        self.modifiers.as_ref()
    }

    fn down(&self, key: &Self::Key) -> bool {
        self.keys_down.iter().any(|k| k == key)
    }

    fn pressed(&self, key: &Self::Key) -> bool {
        self.keys_pressed.iter().any(|k| k == key)
    }

    fn released(&self, key: &Self::Key) -> bool {
        self.keys_released.iter().any(|k| k == key)
    }

    fn text(&self) -> &str {
        &self.text_buffer
    }

    fn clear_presses(&mut self) -> &mut Self {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.text_buffer_builder = SmolStrBuilder::default();
        self.text_buffer = SmolStr::default();
        self
    }

    fn press(&mut self, key: Self::Key) -> &mut Self {
        if !self.down(&key) {
            self.keys_down.push(key.clone());
        }
        if !self.pressed(&key) {
            self.keys_pressed.push(key);
        }
        self
    }

    fn release(&mut self, key: Self::Key) -> &mut Self {
        self.keys_down.retain(|k| k != &key);
        if !self.released(&key) {
            self.keys_released.push(key);
        }
        self
    }

    fn set_modifiers(&mut self, modifiers: Self::Mods) -> &mut Self {
        self.modifiers = Some(modifiers);
        self
    }

    fn receive_text<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
        self.text_buffer_builder.push_str(text.as_ref());
        self.text_buffer = self.text_buffer_builder.finish();
        self
    }

    fn receive_char(&mut self, ch: char) -> &mut Self {
        self.text_buffer_builder.push(ch);
        self.text_buffer = self.text_buffer_builder.finish();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_not_pressed_or_released_by_default() {
        let keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        assert!(!keyboard.down(&10));
        assert!(!keyboard.pressed(&10));
        assert!(!keyboard.released(&10));
    }

    #[test]
    fn key_down_when_pressed() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        keyboard.press(10);
        assert!(keyboard.down(&10));
    }

    #[test]
    fn key_not_down_when_released() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        keyboard.press(10).release(10);
        assert!(!keyboard.down(&10));
    }

    #[test]
    fn key_pressed_after_pressing() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        keyboard.press(10);
        assert!(keyboard.pressed(&10));
    }

    #[test]
    fn key_released_after_releasing() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        keyboard.release(10);
        assert!(keyboard.released(&10));
    }

    #[test]
    fn key_can_be_pressed_and_released_on_same_frame() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        keyboard.press(10).release(10);
        assert!(keyboard.pressed(&10));
        assert!(keyboard.released(&10));
    }

    #[test]
    fn key_pressed_resets_at_start_of_frame() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        keyboard.press(10);
        keyboard.clear_presses();
        assert!(!keyboard.pressed(&10));
    }

    #[test]
    fn key_released_resets_at_start_of_frame() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        keyboard.release(10);
        keyboard.clear_presses();
        assert!(!keyboard.pressed(&10));
    }

    #[test]
    fn key_down_persists_across_frames() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        keyboard.press(10);
        keyboard.clear_presses();
        assert!(keyboard.down(&10));
    }

    #[test]
    fn modifiers_empty_by_default() {
        let keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        assert_eq!(keyboard.modifiers(), None);
    }

    #[test]
    fn can_set_modifiers() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        keyboard.set_modifiers(Modifiers {
            ctrl: true,
            alt: true,
            shift: true,
            logo: true,
        });

        assert_eq!(
            keyboard.modifiers(),
            Some(&Modifiers {
                ctrl: true,
                alt: true,
                shift: true,
                logo: true,
            })
        )
    }

    #[test]
    fn modifiers_persisit_over_frames() {
        let mut keyboard: Keyboard<usize, Modifiers> = Keyboard::new();
        keyboard.set_modifiers(Modifiers {
            ctrl: true,
            alt: true,
            shift: true,
            logo: true,
        });

        keyboard.clear_presses();
        assert_eq!(
            keyboard.modifiers(),
            Some(&Modifiers {
                ctrl: true,
                alt: true,
                shift: true,
                logo: true,
            })
        )
    }
}
