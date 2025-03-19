use crate::prelude::*;

pub struct Projection {
    aspect: f32,
    fovy: f32, // radians
    znear: f32,
    zfar: f32
}

impl Projection {
    pub fn new(
        width: u32,
        height: u32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar
        }
    }

    pub fn std(config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            aspect: config.width as f32 / config.height as f32,
            fovy: 45_f32.to_radians(),
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar)
    }
}
