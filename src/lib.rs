pub mod application;
pub mod datatypes;
pub mod camera;
pub mod utils;
pub mod buffers;
pub mod binds;

pub mod prelude {
    pub use std::sync::{
        Arc,
        Mutex,
    };  
    pub use std::mem::size_of;

    pub use winit::{
        event::*,
        event_loop::{ControlFlow, EventLoop, ActiveEventLoop},
        application::ApplicationHandler,
        window::{Window, WindowId, WindowAttributes},
        keyboard::*,
    };

    pub use wgpu::{
        util::DeviceExt,
    };

    pub use glam::{
        Mat4,
        Vec4,
        Vec3,
        Vec2,
        vec2,
        vec3,
        vec4,
        mat4,

        Quat,
    };

    pub use bytemuck;
    pub use anyhow::*;

    pub use pollster::block_on;
    
    pub use crate::application::prelude::*;
    pub use crate::datatypes::prelude::*;
    pub use crate::camera::prelude::*;
    pub use crate::utils::prelude::*;
    pub use crate::buffers::prelude::*;
    pub use crate::binds::prelude::*;
}
