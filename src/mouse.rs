use crate::Event;
use std::ops::Add;

/// A structure representing the current state of a mouse.
#[derive(Debug, Clone)]
pub struct Mouse<Button, Coord>
where
    Button: Copy + PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    position: [Coord; 2],
    buttons_down: Vec<Button>,
    buttons_pressed: Vec<Button>,
    buttons_released: Vec<Button>,
}

impl<Button, Coord> Default for Mouse<Button, Coord>
where
    Button: Copy + PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    fn default() -> Self {
        Mouse {
            position: Default::default(),
            buttons_down: Vec::with_capacity(4),
            buttons_pressed: Vec::with_capacity(4),
            buttons_released: Vec::with_capacity(4),
        }
    }
}

impl<Button, Coord> Mouse<Button, Coord>
where
    Button: Copy + PartialEq,
    Coord: Copy + Default + Add<Output = Coord>,
{
    pub fn new() -> Self {
        Self::default()
    }

    /// Create the Mouse at a specific pointer position.
    pub fn at_position(position: [Coord; 2]) -> Self {
        Mouse {
            position,
            ..Default::default()
        }
    }

    /// Returns the position of the mouse pointer.
    pub fn position(&self) -> [Coord; 2] {
        self.position
    }

    /// Returns `true` if the given button is currently held down.
    pub fn down(&self, button: Button) -> bool {
        self.buttons_down.iter().any(|&b| b == button)
    }

    /// Returns `true` if the given button was pressed this frame.
    pub fn pressed(&self, button: Button) -> bool {
        self.buttons_pressed.iter().any(|&b| b == button)
    }

    /// Returns `true` if the given button was released this frame.
    pub fn released(&self, button: Button) -> bool {
        self.buttons_released.iter().any(|&b| b == button)
    }

    /// Clears the pressed state of held buttons. Should be called at end of frame.
    pub fn clear_presses(&mut self) -> &mut Self {
        self.buttons_pressed.clear();
        self.buttons_released.clear();
        self
    }

    /// Set the position of the mouse to the given value.
    pub fn move_to(&mut self, position: [Coord; 2]) -> &mut Self {
        self.position = position;
        self
    }

    /// Modify the position of the mouse by the given offset.
    pub fn move_by(&mut self, [x, y]: [Coord; 2]) -> &mut Self {
        let [ox, oy] = self.position;
        self.position = [ox + x, oy + y];
        self
    }

    /// Register that a button was pressed down.
    pub fn press(&mut self, button: Button) -> &mut Self {
        if !self.down(button) {
            self.buttons_down.push(button);
        }
        if !self.pressed(button) {
            self.buttons_pressed.push(button);
        }
        self
    }

    /// Register that a button was released.
    pub fn release(&mut self, button: Button) -> &mut Self {
        self.buttons_down.retain(|&b| b != button);
        if !self.released(button) {
            self.buttons_released.push(button);
        }
        self
    }

    /// Convenience method for handling events. The type of event, `E`, will
    /// vary depending on the windowing library being used.
    pub fn handle_event<E: Event<Self>>(&mut self, event: &E) -> &mut Self {
        event.handle(self);
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
