#[cfg(feature = "winit-support")]
extern crate winit;

#[cfg(feature = "winit-support")]
pub mod winit_support;

mod keyboard;
mod mouse;

pub use crate::keyboard::{Keyboard, KeyboardInput};
pub use crate::mouse::{Mouse, MouseInput};

pub trait Event<Handler> {
    fn handle(&self, handler: &mut Handler);
}
