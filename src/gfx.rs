use cgmath::{Point3, Vector3, InnerSpace};
use glium::{
    backend::glutin::DisplayCreationError,
    glutin::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    Display, VertexBuffer, index::{IndexBuffer, PrimitiveType}};

pub fn new(
    title: &str,
    size: LogicalSize<f64>,
) -> Result<(Display, EventLoop<()>), DisplayCreationError> {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_inner_size(size).with_title(title);
    let cb = ContextBuilder::new().with_depth_buffer(24);
    Ok((Display::new(wb, cb, &event_loop)?, event_loop))
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);

impl From<Vector3<f32>> for Vertex {
    fn from(v: Vector3<f32>) -> Self {
        Vertex { position: v.into() }
    }
}

impl From<Point3<f32>> for Vertex {
    fn from(v: Point3<f32>) -> Self {
        Vertex { position: v.into() }
    }
}

pub struct Mesh {
  verticies: Vec<Point3<f32>>,
  indices: Vec<u32>,
  buffer: Option<VertexBuffer<Vertex>>,
  index_buffer: Option<IndexBuffer<u32>>
}

impl Mesh {
  pub fn new() -> Self {
    Mesh {
      verticies: vec![],
      indices: vec![],
      buffer: None,
      index_buffer: None,
    }
  }

  pub fn cube() -> Self {
    let mut mesh = Mesh::new();
    // Front
    mesh.add_face(&[
      Point3::new(0.0, 0.0, 0.0),
      Point3::new(1.0, 0.0, 0.0),
      Point3::new(1.0, 1.0, 0.0),
    ]);
    mesh.add_face(&[
      Point3::new(0.0, 0.0, 0.0),
      Point3::new(1.0, 1.0, 0.0),
      Point3::new(0.0, 1.0, 0.0),
    ]);

    // Left
    mesh.add_face(&[
      Point3::new(0.0, 0.0, 0.0),
      Point3::new(0.0, 1.0, 0.0),
      Point3::new(0.0, 0.0, 1.0),
    ]);
    mesh.add_face(&[
      Point3::new(0.0, 1.0, 0.0),
      Point3::new(0.0, 1.0, 1.0),
      Point3::new(0.0, 0.0, 1.0),
    ]);

    // Right
    mesh.add_face(&[
      Point3::new(1.0, 0.0, 0.0),
      Point3::new(1.0, 0.0, 1.0),
      Point3::new(1.0, 1.0, 0.0),
    ]);
    mesh.add_face(&[
      Point3::new(1.0, 1.0, 0.0),
      Point3::new(1.0, 0.0, 1.0),
      Point3::new(1.0, 1.0, 1.0),
    ]);

    // Back
    mesh.add_face(&[
      Point3::new(1.0, 0.0, 1.0),
      Point3::new(0.0, 0.0, 1.0),
      Point3::new(0.0, 1.0, 1.0),
    ]);
    mesh.add_face(&[
      Point3::new(0.0, 1.0, 1.0),
      Point3::new(1.0, 1.0, 1.0),
      Point3::new(1.0, 0.0, 1.0),
    ]);

    // Top
    mesh.add_face(&[
      Point3::new(0.0, 1.0, 0.0),
      Point3::new(1.0, 1.0, 0.0),
      Point3::new(1.0, 1.0, 1.0),
    ]);
    mesh.add_face(&[
      Point3::new(1.0, 1.0, 1.0),
      Point3::new(0.0, 1.0, 1.0),
      Point3::new(1.0, 1.0, 0.0),
    ]);

    // Bottom
    mesh.add_face(&[
      Point3::new(0.0, 0.0, 0.0),
      Point3::new(0.0, 0.0, 1.0),
      Point3::new(1.0, 0.0, 0.0),
    ]);
    mesh.add_face(&[
      Point3::new(0.0, 0.0, 1.0),
      Point3::new(1.0, 0.0, 1.0),
      Point3::new(1.0, 0.0, 0.0),
    ]);

    mesh
  }

  pub fn add_face(&mut self, face: &[Point3<f32>]) {
    if face.len() < 3 {
      return;
    }

    let mut indices = [0 as u32; 3];

    for (ind, v) in self.verticies.iter().enumerate() {
      for f in 0..3 {
        let dist = (v - face[f]).magnitude();
        if dist < 1e-5 {
          indices[f] = 1 + ind as u32;
        }
      }
    }

    for f in 0..3 {
      if indices[f] == 0 {
        self.indices.push(self.verticies.len() as u32);
        self.verticies.push(face[f]);
      } else {
        self.indices.push(indices[f] - 1);
      }
    }

  }

  pub fn upload(&mut self, disp: &Display) -> Result<(), Box<dyn std::error::Error>> {
    let shape: Vec<Vertex> = self.verticies.iter().copied().map(Into::into).collect();
    self.buffer = Some(VertexBuffer::new(disp, &shape)?);
    self.index_buffer = Some(IndexBuffer::new(disp, PrimitiveType::TrianglesList, &self.indices)?);
    Ok(())
  }

  pub fn vbo(&self) -> &Option<VertexBuffer<Vertex>> {
    &self.buffer
  }

  pub fn ibo(&self) -> &Option<IndexBuffer<u32>> {
    &self.index_buffer
  }
}
