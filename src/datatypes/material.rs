use crate::prelude::Texture;

pub struct Material {
    pub name: String,
    pub diffuse_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(name: String, diffuse_texture: Texture, bind_group: wgpu::BindGroup) -> Self {
        Self {
            name,
            diffuse_texture,
            bind_group,
        }
    }
}
