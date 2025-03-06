use crate::prelude::*;

pub struct CameraController {
    speed: f32,
    acceleration: f32,
    turn: f32,
    look_change: Vec3,
    direction_change: Vec3,
    velocity: Vec3,
    drone: bool,
}

impl CameraController {
    pub fn new(drone: bool) -> Self {
        Self {
            speed: 0.001,
            acceleration: 0.1,
            turn: 0.005,
            direction_change: Vec3::ZERO,
            look_change: Vec3::ZERO,
            velocity: Vec3::ZERO,
            drone,
        } 
    }

    pub fn process_events(&mut self, input: &Input) {
        if input.held(KeyCode::ArrowUp) {
            self.look_change.y += 1.
        }
        if input.held(KeyCode::ArrowDown) {
            self.look_change.y -= 1.
        }
        if input.held(KeyCode::ArrowLeft) {
            self.look_change.x -= 1.
        }
        if input.held(KeyCode::ArrowRight) {
            self.look_change.x += 1.
        }

        self.look_change = self.look_change.normalize_or_zero();

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

        self.direction_change = self.direction_change.normalize_or_zero();
    }

    pub fn update(&mut self, camera: &mut Camera) {
        let direction = (camera.target - camera.eye).normalize_or_zero();
        let right = (camera.world_up.cross(direction)).normalize_or_zero();
        let camera_up = direction.cross(right);

        if self.drone {
            // adjust velocity by direction_change x acc to a max of speed 
            let time = millis() as f32 / 1000.;
            camera.eye.x = f32::sin(time) * 3.0; 
            camera.eye.z = f32::cos(time) * 3.0;
            println!("{:?}", time);
        }
        else {
            camera.eye += self.direction_change * self.speed;
            camera.target += self.direction_change * self.speed;
            camera.target += self.look_change * self.turn;
        }

        self.direction_change = Vec3::ZERO;
        self.look_change = Vec3::ZERO;
    }
}
