use crate::prelude::*;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32, // radians
    pub pitch: f32, // radians
    pub projection: Projection,
}

impl Camera {
    pub const WORLD_UP: Vec3 = Vec3::Y;

    pub fn new(position: Vec3, yaw: f32, pitch: f32, projection: Projection) -> Self {
        Self {
            position,
            yaw,
            pitch,
            projection,
        }
    }

    pub fn std(config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            position: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,
            projection: Projection::std(config),
        }
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.position,
            Vec3::new(
                self.pitch.cos() * self.yaw.cos(),
                self.pitch.sin(),
                self.pitch.cos() * self.yaw.sin()
            ) + self.position,
            Camera::WORLD_UP,
        )
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.matrix()
    }
}


