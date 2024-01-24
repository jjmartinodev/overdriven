use overdriven::{self, line::LineRenderer, Context};
use winit::{event::{Event, VirtualKeyCode, WindowEvent}, event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut ctx = Context::blocked_new(&window);
    let mut renderer = LineRenderer::new(&ctx);

    let mut lines: Vec<((f32,f32,f32,f32),(f32,f32,f32,f32))> = vec![];

    _ = event_loop.run(move |event, _, elwt| {
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(..) => {
                for line in &mut lines {
                    if line.0.0 < -1. || line.0.0 > 1. {line.1.0 *= -1.}
                    if line.0.1 < -1. || line.0.1 > 1. {line.1.1 *= -1.}
                    if line.0.2 < -1. || line.0.2 > 1. {line.1.2 *= -1.}
                    if line.0.3 < -1. || line.0.3 > 1. {line.1.3 *= -1.}
                    line.0.0 += line.1.0;
                    line.0.1 += line.1.1;
                    line.0.2 += line.1.2;
                    line.0.3 += line.1.3;
                }
                for (line, _) in &lines {
                    renderer.line(line.0, line.1, line.2, line.3)
                }
                renderer.render(&ctx);
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => elwt.set_exit(),
                    WindowEvent::Resized(new_size) => ctx.resize(new_size),
                    WindowEvent::KeyboardInput { input, .. } => {
                        match input.virtual_keycode.unwrap() {
                            VirtualKeyCode::Key0 => {
                                lines.push(((0.,0.,0.,0.),(-0.0011,-0.0001,0.0011,0.0001)));
                            },
                            _ => ()
                        }
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    });
}