use crate::prelude::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CameraUniform {
    view_pos: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

unsafe impl bytemuck::Pod for CameraUniform {}
unsafe impl bytemuck::Zeroable for CameraUniform {}
impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_pos: [0.0; 4],
            view_proj: Mat4::IDENTITY.to_cols_array_2d() 
        }
    }

    pub fn from_proj(camera: &Camera) -> Self {
        Self {
            view_pos: Vec4::new(camera.position.x, camera.position.y, camera.position.z, 1.0).to_array(),
            view_proj: (camera.projection_matrix() * camera.matrix()).to_cols_array_2d(),
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        self.view_proj = (camera.projection_matrix() * camera.matrix()).to_cols_array_2d();
    }
}


