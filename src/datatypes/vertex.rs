// use crate::prelude::*;

pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}
