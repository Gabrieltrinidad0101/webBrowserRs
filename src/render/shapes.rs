use glium::glutin::surface::WindowSurface;
use glium::{implement_vertex, Display, VertexBuffer};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub struct Square {
    pub position: Vec<Vertex>,
    pub color: [f32; 4],
    pub vertex_buffer: VertexBuffer<Vertex>,
}

pub struct Shapes  {
    pub shapes: Vec<Square>,
}


impl Shapes {
    pub fn new() -> Self {
        Self {  
            shapes: Vec::new()
        }
    }


    pub fn square(&mut self,display: &Display<WindowSurface>, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        let position = Self::position(x, y, w, h);
        let vertex_buffer = VertexBuffer::new(display, &position).unwrap();
        self.shapes.push(Square {
            position,
            color,
            vertex_buffer,
        });
    }


    fn position(x: f32, y: f32, w: f32, h: f32) -> Vec<Vertex> {
        vec![
            Vertex { position: [x,     y] },
            Vertex { position: [x + w, y] },
            Vertex { position: [x,  y + h] },
            Vertex { position: [x + w, y + h] },
        ]
    }
}
