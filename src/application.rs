pub mod app;
pub mod state;

pub mod prelude {
    pub use crate::application::app::App;
    pub use crate::application::state::State;
}
