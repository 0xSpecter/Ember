use crate::prelude::*;

pub trait CameraController {
    fn process_events(&mut self, input: &Input);
    fn update(&mut self, camera: &mut Camera, delta: f32);
}


