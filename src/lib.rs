//! This is simple Rust crate for managing and querying input state.
//!
//! It treats the mouse and keyboard as an immutable data structure that you
//! can query to find which keys and buttons are pressed (or where the pointer
//! is).
//!
//! The core data structures of `Mouse` and `Keyboard` are generic, in theory
//! supporting multiple windowing libraries. You can roll your own, or you can
//! enable the `winit` which has factory methods to easily create
//! a `Mouse` and `Keyboard` which work with the `winit` library.
//!
//! As stated, the mouse and keyboard are immutable. To track input changes,
//! each provide a `begin_frame_input` method which return an object you can
//! make changes to for the frame.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #[cfg(feature = "winit_0_21")]
//! # use winit_0_21 as winit;
//!
//! # #[cfg(feature = "winit_0_24")]
//! # use winit_0_24 as winit;
//!
//! # #[cfg(feature = "winit_0_27")]
//! # use winit_0_27 as winit;
//! # use winit::event::{Event, VirtualKeyCode, MouseButton};
//! let mut event_loop = winit::event_loop::EventLoop::new();
//! let mut keyboard = buttons::support::winit::keyboard();
//! let mut mouse = buttons::support::winit::mouse();
//! let mut touch = buttons::support::winit::touch();
//!
//! // Track input
//! event_loop.run(move |event, _, _| {
//!     keyboard.handle_event(&event);
//!     mouse.handle_event(&event);
//!     touch.handle_event(&event);
//!
//!     // Check state
//!     if keyboard.pressed(VirtualKeyCode::Escape)
//!         || mouse.released(MouseButton::Right)
//!         || touch.first_touch().is_some()
//!     {
//!         // Do something
//!     }
//! });
//! ```

pub mod support;

mod keyboard;
mod mouse;
mod touch;

pub use crate::keyboard::Keyboard;
pub use crate::mouse::Mouse;
pub use crate::touch::Touchpad;

/// A trait for events that can modify input state.
pub trait Event<Handler> {
    /// Modify the state of the provided handler (for example, a `Mouse` or
    /// `Keyboard`).
    ///
    /// This should rarely be called directly. Instead, pass this event to
    /// the handler's `handle_event` method.
    fn handle(&self, handler: &mut Handler);
}
