use crate::prelude::*;

#[derive(Default)]
pub struct DroneCam {
    max_speed: f32,
    acceleration: f32, // 0.1 is 10 ticks to full speed
    direction_change: Vec3,
    look_change: Vec2,
    velocity: Vec3,
    drone_look_speed: f32,
}

impl CameraController for DroneCam {
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

        if input.held(KeyCode::ArrowLeft) {
            self.look_change.x -= 1.
        }
        if input.held(KeyCode::ArrowRight) {
            self.look_change.x += 1.
        }
        if input.held(KeyCode::ArrowUp) {
            self.look_change.y += 1.
        }
        if input.held(KeyCode::ArrowDown) {
            self.look_change.y -= 1.
        }
    }

    fn update(&mut self, camera: &mut Camera, delta: f32) {
        let yc = camera.yaw.cos();
        let ys = camera.yaw.sin();
        let forward = Vec3::new(yc, 0.0, ys).normalize_or_zero();
        let right = Vec3::new(-ys, 0.0, yc).normalize_or_zero();

        self.velocity += self.acceleration * self.direction_change;
        if self.velocity.x.abs() > 0. && self.direction_change.x != 1_f32.copysign(self.velocity.y) {self.velocity.x -= self.acceleration.copysign(self.velocity.x) / 2.}
        if self.velocity.y.abs() > 0. && self.direction_change.y != 1_f32.copysign(self.velocity.y) {self.velocity.y -= self.acceleration.copysign(self.velocity.y) / 2.}
        if self.velocity.z.abs() > 0. && self.direction_change.z != 1_f32.copysign(self.velocity.y) {self.velocity.z -= self.acceleration.copysign(self.velocity.z) / 2.}
        self.velocity = self.velocity.clamp(Vec3::new(-self.max_speed, -self.max_speed, -self.max_speed), Vec3::new(self.max_speed, self.max_speed, self.max_speed));

        camera.position += -forward * self.velocity.z * delta;
        camera.position += right *  self.velocity.x * delta;

        camera.position += Vec3::Y * self.velocity.y * delta;

        camera.yaw += self.look_change.x.to_radians() * self.drone_look_speed * delta;
        camera.pitch += self.look_change.y.to_radians() * self.drone_look_speed * delta;

        self.direction_change = Vec3::ZERO;
        self.look_change = Vec2::ZERO;
    }
}

impl DroneCam {
    pub fn new() -> Self {
        Self {
            max_speed: 20.0,
            acceleration: 0.05,
            direction_change: Vec3::ZERO,
            look_change: Vec2::ZERO,
            velocity: Vec3::ZERO,
            drone_look_speed: 100.,
        } 
    }
}
