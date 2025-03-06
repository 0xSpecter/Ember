use crate::prelude::*;

#[derive(Default, Debug)]
pub struct Input {
    pressed_keys: Vec<KeyCode>,
    held_keys: Vec<KeyCode> 
}

impl Input {
    pub fn new() -> Self {
        Self {
            held_keys: vec![],
            pressed_keys: vec![], 
        }
    }

    pub fn add_key(&mut self, key_event: KeyEvent) {
        if let PhysicalKey::Code(key) = key_event.physical_key {
            if key_event.state == ElementState::Released {
                if let Some(index) = self.held_keys.iter().position(|hkey| *hkey == key) {
                    self.held_keys.swap_remove(index);
                }
            }
            else if !key_event.repeat {
                self.push(key);
            }
        }
        else {
            panic!("Could not add key:  {:?}", key_event);
        }
    }

    fn push(&mut self, key: KeyCode) {
        self.pressed_keys.push(key);
        self.held_keys.push(key);
    }

    pub fn held(&self, key: KeyCode) -> bool {
        self.held_keys.contains(&key)
    }

    pub fn pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn update(&mut self) {
        self.pressed_keys.clear(); 
    }
}
