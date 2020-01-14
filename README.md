buttons
===

A simple Rust crate for managing and querying input state.

[![Build Status](https://travis-ci.org/mistodon/buttons.svg?branch=master)](https://travis-ci.org/mistodon/buttons)
[![Crates.io](https://img.shields.io/crates/v/buttons.svg)](https://crates.io/crates/buttons)
[![Docs.rs](https://docs.rs/buttons/badge.svg)](https://docs.rs/buttons/0.1.1/buttons/)

# Usage

## With `winit`

(Enabling the `winit-support` feature.)

```rust
let mut keyboard = buttons::winit_support::keyboard();
let mut mouse = buttons::winit_support::mouse();

{
    let mut keyboard_input = keyboard.begin_frame_input();
    let mut mouse_input = mouse.begin_frame_input();

    events_loop.poll_events(|event| {
        if let Event::WindowEvent { event, .. } = event {
            keyboard_input.handle_event(&event);
            mouse_input.handle_event(&event);
        }
    });
}

if keyboard.pressed(VirtualKeyCode::Escape) || mouse.released(MouseButton::Right) {
    ...
}
```
