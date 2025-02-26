buttons
===

[![Crates.io](https://img.shields.io/crates/v/buttons.svg)](https://crates.io/crates/buttons)
[![Docs.rs](https://docs.rs/buttons/badge.svg)](https://docs.rs/buttons/0.6.0/buttons/)

A simple Rust crate for managing and querying input state.

# Usage

## With `winit`

(Enabling the `winit` feature.)

```rust
let mut event_loop = winit::event_loop::EventLoop::new();
let mut keyboard = buttons::winit::keyboard();
let mut mouse = buttons::winit::mouse();
let mut touch = buttons::winit::touch();

// Track input
event_loop.run(move |event, _, _| {
    keyboard.handle_event(&event);
    mouse.handle_event(&event);
    touch.handle_event(&event);

    // Check state
    if keyboard.pressed(VirtualKeyCode::Escape)
        || mouse.released(MouseButton::Right)
        || touch.first_touch().is_some()
    {
        // Do something
    }
});
```
