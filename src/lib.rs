#![allow(unknown_lints)]

mod keyboard;
mod mouse;

pub use keyboard::{KeyId, Keyboard, KeyboardInput, Modifiers};
pub use mouse::{Mouse, MouseButtonId, MouseInput};
