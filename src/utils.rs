pub mod input;
pub mod time;
pub mod files;
pub mod math;

pub mod prelude {
    pub use crate::utils::input::Input;
    pub use crate::utils::time::millis;
    pub use crate::utils::files::{
        read_file,
        stuff_path,
    };
    pub use crate::utils::math::{
        log_interpolate,
        lerp,
        lerp_vec3,
    };
}
