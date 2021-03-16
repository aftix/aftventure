use cgmath::Vector4;
use glium::{
    backend::glutin::DisplayCreationError,
    glutin::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    Display,
};

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
    pub position: [f32; 4],
}
implement_vertex!(Vertex, position);

impl From<Vector4<f32>> for Vertex {
    fn from(v: Vector4<f32>) -> Self {
        Vertex { position: v.into() }
    }
}
