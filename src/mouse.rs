use crate::Event;

use smallvec::SmallVec;

use std::ops::Add;

/// A trait for objects that can represent the state of a mouse.
pub trait MouseInterface {
    /// A type representing a mouse button.
    type Button;

    /// The numeric type used for pointer coordinates.
    type Coord;

    /// Returns the position of the mouse pointer.
    fn position(&self) -> [Self::Coord; 2];

    /// Returns `true` if the given button is currently held down.
    fn down(&self, button: Self::Button) -> bool;

    /// Returns `true` if the given button was pressed this frame.
    fn pressed(&self, button: Self::Button) -> bool;

    /// Returns `true` if the given button was released this frame.
    fn released(&self, button: Self::Button) -> bool;

    /// Clears the pressed state of held buttons. Should be called at end of frame.
    fn clear_presses(&mut self) -> &mut Self;

    /// Set the position of the mouse to the given value.
    fn move_to(&mut self, position: [Self::Coord; 2]) -> &mut Self;

    /// Modify the position of the mouse by the given offset.
    fn move_by(&mut self, delta_position: [Self::Coord; 2]) -> &mut Self;

    /// Register that a button was pressed down.
    fn press(&mut self, button: Self::Button) -> &mut Self;

    /// Register that a button was released.
    fn release(&mut self, button: Self::Button) -> &mut Self;

    /// Convenience method for handling events. The type of event, `E`, will
    /// vary depending on the windowing library being used.
    fn handle_event<E: Event<Self>>(&mut self, event: &E) -> &mut Self {
        event.handle(self);
        self
    }
}

/// A structure representing the current state of a mouse.
#[derive(Debug, Clone)]
pub struct Mouse<Button, Coord>
where
    Button: Copy + PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    position: [Coord; 2],
    buttons_down: SmallVec<[Button; 4]>,
    buttons_pressed: SmallVec<[Button; 4]>,
    buttons_released: SmallVec<[Button; 4]>,
}

impl<Button, Coord> Default for Mouse<Button, Coord>
where
    Button: Copy + PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Button, Coord> Mouse<Button, Coord>
where
    Button: Copy + PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    pub fn new() -> Self {
        Mouse {
            position: Default::default(),
            buttons_down: Default::default(),
            buttons_pressed: Default::default(),
            buttons_released: Default::default(),
        }
    }

    /// Create the Mouse at a specific pointer position.
    pub fn at_position(position: [Coord; 2]) -> Self {
        Mouse {
            position,
            ..Default::default()
        }
    }
}

impl<B, C> MouseInterface for Mouse<B, C>
where
    B: Copy + PartialEq,
    C: Copy + Default + Add<Output = C>,
{
    type Button = B;
    type Coord = C;

    fn position(&self) -> [Self::Coord; 2] {
        self.position
    }

    fn down(&self, button: Self::Button) -> bool {
        self.buttons_down.iter().any(|&b| b == button)
    }

    fn pressed(&self, button: Self::Button) -> bool {
        self.buttons_pressed.iter().any(|&b| b == button)
    }

    fn released(&self, button: Self::Button) -> bool {
        self.buttons_released.iter().any(|&b| b == button)
    }

    fn clear_presses(&mut self) -> &mut Self {
        self.buttons_pressed.clear();
        self.buttons_released.clear();
        self
    }

    fn move_to(&mut self, position: [Self::Coord; 2]) -> &mut Self {
        self.position = position;
        self
    }

    fn move_by(&mut self, [x, y]: [Self::Coord; 2]) -> &mut Self {
        let [ox, oy] = self.position;
        self.position = [ox + x, oy + y];
        self
    }

    fn press(&mut self, button: Self::Button) -> &mut Self {
        if !self.down(button) {
            self.buttons_down.push(button);
        }
        if !self.pressed(button) {
            self.buttons_pressed.push(button);
        }
        self
    }

    fn release(&mut self, button: Self::Button) -> &mut Self {
        self.buttons_down.retain(|b| b != &button);
        if !self.released(button) {
            self.buttons_released.push(button);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mouse_has_no_button_state() {
        let mouse: Mouse<usize, f64> = Mouse::new();
        assert!(!mouse.down(0));
        assert!(!mouse.pressed(0));
        assert!(!mouse.released(0));
    }

    #[test]
    fn default_mouse_is_at_zero_position() {
        let mouse: Mouse<usize, f64> = Mouse::new();
        assert_eq!(mouse.position(), [0.0, 0.0]);
    }

    #[test]
    fn mouse_can_be_created_at_a_position() {
        let mouse: Mouse<usize, f64> = Mouse::at_position([100.0, 100.0]);
        assert_eq!(mouse.position(), [100.0, 100.0]);
    }

    #[test]
    fn mouse_can_be_placed() {
        let mut mouse: Mouse<usize, f64> = Mouse::new();
        mouse.move_to([100.0, 100.0]);
        assert_eq!(mouse.position(), [100.0, 100.0]);
    }

    #[test]
    fn mouse_can_be_moved() {
        let mut mouse: Mouse<usize, f64> = Mouse::at_position([1.0, 1.0]);
        mouse.move_by([-1.0, -1.0]);
        assert_eq!(mouse.position(), [0.0, 0.0]);
    }

    #[test]
    fn mouse_button_down_when_pressed() {
        let mut mouse: Mouse<usize, f64> = Mouse::new();
        mouse.press(1);
        assert!(mouse.down(1));
    }

    #[test]
    fn mouse_button_not_down_when_released() {
        let mut mouse: Mouse<usize, f64> = Mouse::new();
        mouse.press(1).release(1);
        assert!(!mouse.down(1));
    }

    #[test]
    fn mouse_button_pressed_after_pressing() {
        let mut mouse: Mouse<usize, f64> = Mouse::new();
        mouse.press(1);
        assert!(mouse.pressed(1));
    }

    #[test]
    fn mouse_button_released_after_releasing() {
        let mut mouse: Mouse<usize, f64> = Mouse::new();
        mouse.release(1);
        assert!(mouse.released(1));
    }

    #[test]
    fn mouse_button_can_be_pressed_and_released_on_same_frame() {
        let mut mouse: Mouse<usize, f64> = Mouse::new();
        mouse.press(1).release(1);
        assert!(mouse.pressed(1));
        assert!(mouse.released(1));
    }

    #[test]
    fn mouse_button_pressed_resets_at_start_of_frame() {
        let mut mouse: Mouse<usize, f64> = Mouse::new();
        mouse.press(1);
        mouse.clear_presses();
        assert!(!mouse.pressed(1));
    }

    #[test]
    fn mouse_button_released_resets_at_start_of_frame() {
        let mut mouse: Mouse<usize, f64> = Mouse::new();
        mouse.release(1);
        mouse.clear_presses();
        assert!(!mouse.pressed(1));
    }

    #[test]
    fn mouse_button_down_persists_across_frames() {
        let mut mouse: Mouse<usize, f64> = Mouse::new();
        mouse.press(1);
        mouse.clear_presses();
        assert!(mouse.down(1));
    }
}
