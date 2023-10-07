#[macro_use]
extern crate glium;

#[allow(unused_imports)]
use glium::{glutin, Surface};

mod shape;
mod vertex_shader;
mod fragment_shader;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    
    // Size and position are custom hardcoded values for my screen. DPI seems off as sizes below my screen resolution.
    let window = wb.clone()
        .with_inner_size(glutin::dpi::LogicalSize::new(1387.0, 1387.0))
        .with_position(glutin::dpi::LogicalPosition::new(1173.0, 0.0));
    let context = glutin::ContextBuilder::new();

    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let shape = shape::shape();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    const TARGET_FPS: u64 = 60;

    // Using &String here, not &str as in example.
    let vertex_shader_src = vertex_shader::vertex_shader_src();
    let fragment_shader_src = fragment_shader::fragment_shader_src();

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src.as_str(),
        fragment_shader_src.as_str(),
    None).unwrap();

    let mut t: f32 = 0.0;
    let mut delta: f32 = 0.02;

    // Event loop: Any changes over time are made here, except for animated shaders (I guess).
    event_loop.run(move |event, _, control_flow| {
        let start_time = std::time::Instant::now();

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;

        let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
            true => 1000 / TARGET_FPS - elapsed_time,
            false => 0
        };
        let new_inst = start_time + std::time::Duration::from_millis(wait_millis);

        *control_flow =  glutin::event_loop::ControlFlow::WaitUntil(new_inst);

        t += delta;
        if (t > std::f32::consts::PI) || (t < -std::f32::consts::PI) {
            delta = -delta;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
