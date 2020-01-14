//! This is simple Rust crate for managing and querying input state.
//!
//! It treats the mouse and keyboard as an immutable data structure that you
//! can query to find which keys and buttons are pressed (or where the pointer
//! is).
//!
//! The core data structures of `Mouse` and `Keyboard` are generic, in theory
//! supporting multiple windowing libraries. You can roll your own, or you can
//! enable the `winit-support` which has factory methods to easily create
//! a `Mouse` and `Keyboard` which work with the `winit` library.
//!
//! As stated, the mouse and keyboard are immutable. To track input changes,
//! each provide a `begin_frame_input` method which return an object you can
//! make changes to for the frame.
//!
//! # Examples
//!
//! ```rust
//! # #[cfg(feature = "winit_support")] {
//! # use winit::{Event, VirtualKeyCode, MouseButton};
//! let mut keyboard = buttons::winit_support::keyboard();
//! let mut mouse = buttons::winit_support::mouse();
//!
//! // Track input
//! {
//!     let mut keyboard_input = keyboard.begin_frame_input();
//!     let mut mouse_input = mouse.begin_frame_input();
//!
//!     events_loop.poll_events(|event| {
//!         if let Event::WindowEvent { event, .. } = event {
//!             keyboard_input.handle_event(&event);
//!             mouse_input.handle_event(&event);
//!         }
//!     });
//! }
//!
//! // Check state
//! if keyboard.pressed(VirtualKeyCode::Escape) || mouse.released(MouseButton::Right) {
//!     // Do something
//! }
//! # }
//! ```

#[cfg(feature = "winit-support")]
extern crate winit;

#[cfg(feature = "winit-support")]
pub mod winit_support;

mod keyboard;
mod mouse;

pub use crate::keyboard::{Keyboard, KeyboardInput};
pub use crate::mouse::{Mouse, MouseInput};

/// A trait for events that can modify input state.
pub trait Event<Handler> {
    /// Modify the state of the provided handler (for example, a `Mouse` or
    /// `Keyboard`).
    ///
    /// This should rarely be called directly. Instead, pass this event to
    /// the handler's `handle_event` method.
    fn handle(&self, handler: &mut Handler);
}
