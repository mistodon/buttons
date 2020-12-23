#![cfg(feature = "winit-support")]

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::<()>::new();
    let window_builder = WindowBuilder::new().with_title("buttons");
    let window = window_builder.build(&event_loop).unwrap();
    let mut keyboard = buttons::winit_support::keyboard();
    let mut mouse = buttons::winit_support::mouse();

    event_loop.run(move |event, _, control_flow| {
        keyboard.handle_event(&event);
        mouse.handle_event(&event);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();

                eprintln!(
                    r#"BUTTONS:
modifiers: {:?}
space pressed: {}
space held: {}
space released: {}

left clicked: {}
mouse position: {:?}
"#,
                    keyboard.modifiers(),
                    keyboard.pressed(VirtualKeyCode::Space),
                    keyboard.down(VirtualKeyCode::Space),
                    keyboard.released(VirtualKeyCode::Space),
                    mouse.pressed(MouseButton::Left),
                    mouse.position()
                );

                keyboard.clear_presses();
                mouse.clear_presses();

                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    });
}
