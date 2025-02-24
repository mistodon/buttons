#[cfg(feature = "winit_0_21")]
use winit_0_21 as winit;

#[cfg(feature = "winit_0_24")]
use winit_0_24 as winit;

#[cfg(feature = "winit_0_27")]
use winit_0_27 as winit;

#[cfg(any(feature = "winit_0_21", feature = "winit_0_24", feature = "winit_0_27"))]
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(any(feature = "winit_0_21", feature = "winit_0_24", feature = "winit_0_27"))]
fn main() {
    let event_loop = EventLoop::<()>::new();
    let window_builder = WindowBuilder::new().with_title("buttons");
    let window = window_builder.build(&event_loop).unwrap();
    let mut keyboard = buttons::support::winit::keyboard();
    let mut mouse = buttons::support::winit::mouse();
    let mut touch = buttons::support::winit::touch();

    event_loop.run(move |event, _, control_flow| {
        keyboard.handle_event(&event);
        mouse.handle_event(&event);
        touch.handle_event(&event);

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
text: {}

left clicked: {}
mouse position: {:?}

primary touch: {:?}
"#,
                    keyboard.modifiers(),
                    keyboard.pressed(VirtualKeyCode::Space),
                    keyboard.down(VirtualKeyCode::Space),
                    keyboard.released(VirtualKeyCode::Space),
                    keyboard.text(),
                    mouse.pressed(MouseButton::Left),
                    mouse.position(),
                    touch.first_touch(),
                );

                keyboard.clear_presses();
                mouse.clear_presses();
                touch.clear_taps();

                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    });
}

#[cfg(not(any(feature = "winit_0_21", feature = "winit_0_24", feature = "winit_0_27")))]
fn main() {}
