use crate::prelude::*;

#[derive(Default)]
pub struct FreeCam {
    speed: f32,
    direction_change: Vec3,
    look_change: Vec2,
    sensitivity: f32,
}

impl CameraController for FreeCam {
    fn process_events(&mut self, input: &Input) {
        if input.held(KeyCode::KeyW) {
            self.direction_change.z += -1. 
        } 
        if input.held(KeyCode::KeyS) {
            self.direction_change.z += 1. 
        } 
        if input.held(KeyCode::KeyD) {
            self.direction_change.x += 1. 
        } 
        if input.held(KeyCode::KeyA) {
            self.direction_change.x += -1. 
        } 
        if input.held(KeyCode::Space) {
            self.direction_change.y += 1. 
        } 
        if input.held(KeyCode::ControlLeft) {
            self.direction_change.y += -1.
        } 

        self.look_change = Vec2::new(input.mouse_delta.x, -input.mouse_delta.y);
        self.direction_change = self.direction_change.normalize_or_zero();
    }

    fn update(&mut self, camera: &mut Camera, delta: f32) {
        let yc = camera.yaw.cos();
        let ys = camera.yaw.sin();
        let forward = Vec3::new(yc, 0.0, ys).normalize_or_zero();
        let right = Vec3::new(-ys, 0.0, yc).normalize_or_zero();

        camera.position += -forward * self.direction_change.z * self.speed * delta;
        camera.position += right * self.direction_change.x * self.speed * delta;

        camera.position += Vec3::Y * self.direction_change.y * self.speed * delta;

        camera.yaw += self.look_change.x.to_radians() * self.sensitivity * delta;
        camera.pitch += self.look_change.y.to_radians() * self.sensitivity * delta;

        self.direction_change = Vec3::ZERO;
        self.look_change = Vec2::ZERO;
    }
}

impl FreeCam {
    pub fn new() -> Self {
        Self {
            speed: 10.0,
            direction_change: Vec3::ZERO,
            look_change: Vec2::ZERO,
            sensitivity: 30.,
        } 
    }
}
