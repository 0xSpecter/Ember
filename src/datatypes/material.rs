use crate::prelude::{Texture, TextureBindGroup};

pub struct Material {
    pub name: String,
    pub diffuse_texture: Texture,
}

impl Material {
    pub fn new(name: String, diffuse_texture: Texture) -> Self {
        Self {
            name,
            diffuse_texture, 
        }
    }
}
