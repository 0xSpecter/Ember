use crate::prelude::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

impl ModelVertex {
    pub fn convert(model: tobj::Model, index: usize) -> Self {
        ModelVertex {
            position: [
                model.mesh.positions[index * 3],
                model.mesh.positions[index * 3 + 1],
                model.mesh.positions[index * 3 + 2],
            ],
            tex_coords: [model.mesh.texcoords[index * 2], 1.0 - model.mesh.texcoords[index * 2 + 1]],
            normal: if !model.mesh.normals.is_empty() { [
                model.mesh.normals[index * 3],
                model.mesh.normals[index * 3 + 1],
                model.mesh.normals[index * 3 + 2],
            ] } else { [0.0, 0.0, 0.0] },
        }
    }
}

impl Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ]
        }
    }
}
