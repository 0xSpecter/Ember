use crate::prelude::*;

pub struct Light {
    pub bind: LightBindGroup,
}

impl Light {
    pub fn new(device: &wgpu::Device) -> Self {
        let bind = LightBindGroup::new(device);

        Light {
            bind,
        }
    } 
}
