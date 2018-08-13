#[cfg(feature = "winit-support")]
extern crate winit;

#[cfg(feature = "winit-support")]
pub mod winit_support;

mod keyboard;
mod mouse;

pub use keyboard::{Keyboard, KeyboardInput};
pub use mouse::{Mouse, MouseInput};

pub trait Event<Handler> {
    fn handle(&self, &mut Handler);
}
