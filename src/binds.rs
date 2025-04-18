pub mod bind_group;
pub mod time_bind_group;
pub mod texture_bind_group;

pub mod prelude {
    pub use crate::binds::bind_group::BindGroup;
    pub use crate::binds::time_bind_group::TimeBindGroup;
    pub use crate::binds::texture_bind_group::TextureBindGroup;
}
