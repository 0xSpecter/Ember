use crate::prelude::*;

pub struct Camera {
    pub eye: Vec3, // position
    pub target: Vec3, // look
    pub world_up: Vec3, // local Y up
    pub aspect: f32, // aspect ratio
    pub fovy: f32, // fov based on y axis
    pub znear: f32, // min view distance
    pub zfar: f32, // max view distance
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.world_up);
        let proj = Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);

        proj * view  // REMEMBER THIS IS NOT BOTH WAY WORK PROJ FIRST THE VIEW YOU MORON IDIOT SCUM
    }

    pub fn std(config: &wgpu::SurfaceConfiguration) -> Self {
        Camera {
            eye: vec3(0.0, 0.0, 2.0), 
            target: vec3(0.0, 0.0, 0.0),
            world_up: glam::Vec3::Y,
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }
}


