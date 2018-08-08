pub trait MouseButtonId: Copy {
    fn button_id(self) -> usize;
    fn from_button_id(button_id: usize) -> Option<Self>
    where
        Self: Sized;
}

impl MouseButtonId for usize {
    fn button_id(self) -> usize {
        self
    }

    fn from_button_id(button_id: usize) -> Option<Self> {
        Some(button_id)
    }
}

#[derive(Clone, Default)]
pub struct Mouse {
    position: [f64; 2],
    buttons_down: [bool; 8],
    buttons_pressed: [bool; 8],
    buttons_released: [bool; 8],
}

impl Mouse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn at_position(position: [f64; 2]) -> Self {
        Mouse {
            position,
            ..Default::default()
        }
    }

    pub fn begin_frame_input(&mut self) -> MouseInput {
        self.buttons_pressed = [false; 8];
        self.buttons_released = [false; 8];
        MouseInput { mouse: self }
    }

    pub fn position(&self) -> [f64; 2] {
        self.position
    }

    pub fn down<Button: MouseButtonId>(&self, button: Button) -> bool {
        self.buttons_down[button.button_id()]
    }

    pub fn pressed<Button: MouseButtonId>(&self, button: Button) -> bool {
        self.buttons_pressed[button.button_id()]
    }

    pub fn released<Button: MouseButtonId>(&self, button: Button) -> bool {
        self.buttons_released[button.button_id()]
    }
}

pub struct MouseInput<'a> {
    mouse: &'a mut Mouse,
}

impl<'a> MouseInput<'a> {
    pub fn move_to(&mut self, position: [f64; 2]) -> &mut Self {
        self.mouse.position = position;
        self
    }

    pub fn move_by(&mut self, [x, y]: [f64; 2]) -> &mut Self {
        let [ox, oy] = self.mouse.position;
        self.mouse.position = [ox + x, oy + y];
        self
    }

    pub fn press<Button: MouseButtonId>(&mut self, button: Button) -> &mut Self {
        self.mouse.buttons_down[button.button_id()] = true;
        self.mouse.buttons_pressed[button.button_id()] = true;
        self
    }

    pub fn release<Button: MouseButtonId>(&mut self, button: Button) -> &mut Self {
        self.mouse.buttons_down[button.button_id()] = false;
        self.mouse.buttons_released[button.button_id()] = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mouse_has_no_button_state() {
        let mouse = Mouse::new();
        assert!(!mouse.down(0));
        assert!(!mouse.pressed(0));
        assert!(!mouse.released(0));
    }

    #[test]
    fn default_mouse_is_at_zero_position() {
        let mouse = Mouse::new();
        assert_eq!(mouse.position(), [0.0, 0.0]);
    }

    #[test]
    fn mouse_can_be_created_at_a_position() {
        let mouse = Mouse::at_position([100.0, 100.0]);
        assert_eq!(mouse.position(), [100.0, 100.0]);
    }

    #[test]
    fn mouse_can_be_placed() {
        let mut mouse = Mouse::new();
        {
            mouse.begin_frame_input().move_to([100.0, 100.0]);
        }
        assert_eq!(mouse.position(), [100.0, 100.0]);
    }

    #[test]
    fn mouse_can_be_moved() {
        let mut mouse = Mouse::at_position([1.0, 1.0]);
        {
            mouse.begin_frame_input().move_by([-1.0, -1.0]);
        }
        assert_eq!(mouse.position(), [0.0, 0.0]);
    }

    #[test]
    fn mouse_button_down_when_pressed() {
        let mut mouse = Mouse::new();
        {
            mouse.begin_frame_input().press(1);
        }
        assert!(mouse.down(1));
    }

    #[test]
    fn mouse_button_not_down_when_released() {
        let mut mouse = Mouse::new();
        {
            mouse.begin_frame_input().press(1).release(1);
        }
        assert!(!mouse.down(1));
    }

    #[test]
    fn mouse_button_pressed_after_pressing() {
        let mut mouse = Mouse::new();
        {
            mouse.begin_frame_input().press(1);
        }
        assert!(mouse.pressed(1));
    }

    #[test]
    fn mouse_button_released_after_releasing() {
        let mut mouse = Mouse::new();
        {
            mouse.begin_frame_input().release(1);
        }
        assert!(mouse.released(1));
    }

    #[test]
    fn mouse_button_can_be_pressed_and_released_on_same_frame() {
        let mut mouse = Mouse::new();
        {
            mouse.begin_frame_input().press(1).release(1);
        }
        assert!(mouse.pressed(1));
        assert!(mouse.released(1));
    }

    #[test]
    fn mouse_button_pressed_resets_at_start_of_frame() {
        let mut mouse = Mouse::new();
        {
            mouse.begin_frame_input().press(1);
        }
        {
            mouse.begin_frame_input();
        }
        assert!(!mouse.pressed(1));
    }

    #[test]
    fn mouse_button_released_resets_at_start_of_frame() {
        let mut mouse = Mouse::new();
        {
            mouse.begin_frame_input().release(1);
        }
        {
            mouse.begin_frame_input();
        }
        assert!(!mouse.pressed(1));
    }

    #[test]
    fn mouse_button_down_persists_across_frames() {
        let mut mouse = Mouse::new();
        {
            mouse.begin_frame_input().press(1);
        }
        {
            mouse.begin_frame_input();
        }
        assert!(mouse.down(1));
    }
}
