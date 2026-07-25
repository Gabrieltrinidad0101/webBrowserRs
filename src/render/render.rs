use glium::Surface;
use glium::winit::event::{Event, WindowEvent};
use glium::uniform;
use crate::render::shapes::Shapes;

pub fn run() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("no se pudo crear el event loop");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("webBrowserRs - render")
        .with_inner_size(800, 600)
        .build(&event_loop);

    let mut shapes = Shapes::new();
    shapes.square(&display,10.0, 10.0, 20.0, 20.0, [0.2f32, 0.7, 1.0, 1.0]);
    shapes.square(&display,30.0, 30.0, 20.0, 20.0, [0.2f32, 0.7, 1.0, 1.0]);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader = r#"
        #version 140
        in vec2 position;
        uniform vec2 u_resolution;
        void main() {
            // píxeles -> NDC (-1..1), con origen arriba-izquierda
            vec2 ndc = (position / u_resolution) * 2.0 - 1.0;
            gl_Position = vec4(ndc.x, -ndc.y, 0.0, 1.0);
        }
    "#;

    let fragment_shader = r#"
    #version 140
    out vec4 color;
    uniform vec4 u_color;    // ← viene desde Rust
    void main() {
        color = u_color;
    }
"#;

    let program =
        glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    let mut frame = display.draw();
                    frame.clear_color(0.1, 0.1, 0.1, 1.0);

                    let (width, height) = display.get_framebuffer_dimensions();
                    let resolution = [width as f32, height as f32];

                    for shape in &shapes.shapes {
                        frame
                            .draw(
                                &shape.vertex_buffer,
                                &indices,
                                &program,
                                &uniform! { u_color: shape.color, u_resolution: resolution },
                                &Default::default(),
                            )
                            .unwrap();
                    }
                    frame.finish().unwrap();
                }
                WindowEvent::Resized(size) => display.resize(size.into()),
                _ => (),
            },
            Event::AboutToWait => window.request_redraw(),
            _ => (),
        })
        .unwrap();
}
