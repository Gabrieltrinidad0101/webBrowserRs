// Setup muy básico de OpenGL con glium: abre una ventana y dibuja un cuadrado.
use glium::implement_vertex;
use glium::Surface;
use glium::winit::event::{Event, WindowEvent};

// Un vértice tiene solo su posición 2D en coordenadas normalizadas (-1.0 a 1.0).
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub fn run() {
    // 1. Ventana + contexto OpenGL.
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("no se pudo crear el event loop");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("webBrowserRs - render")
        .with_inner_size(800, 600)
        .build(&event_loop);

    // 2. El cuadrado: 4 vértices dibujados como triangle strip.
    //    v2 --- v3
    //    |    / |
    //    | /    |
    //    v0 --- v1
    let square = vec![
        Vertex { position: [-0.5, -0.5] }, // abajo-izquierda
        Vertex { position: [ 0.5, -0.5] }, // abajo-derecha
        Vertex { position: [-0.5,  0.5] }, // arriba-izquierda
        Vertex { position: [ 0.5,  0.5] }, // arriba-derecha
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &square).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    // 3. Shaders mínimos: el vertex pasa la posición y el fragment pinta de un color fijo.
    let vertex_shader = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.2, 0.7, 1.0, 1.0); // celeste
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

    // 4. Bucle de eventos: limpia el fondo y dibuja el cuadrado en cada frame.
    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    let mut frame = display.draw();
                    frame.clear_color(0.1, 0.1, 0.1, 1.0); // fondo gris oscuro
                    frame
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &glium::uniforms::EmptyUniforms,
                            &Default::default(),
                        )
                        .unwrap();
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
