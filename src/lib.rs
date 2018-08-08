#[cfg(feature = "winit_support")]
extern crate winit;

#[cfg(feature = "winit_support")]
mod winit_support;

mod keyboard;
mod mouse;

pub use keyboard::{KeyId, Keyboard, KeyboardInput, Modifiers};
pub use mouse::{Mouse, MouseButtonId, MouseInput};
