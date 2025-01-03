pub use async_trait::async_trait;
pub use eyre;

pub use app::{handle_error, App};
pub use boot::Hooks;
pub use di::Module;
pub mod boot;
pub mod di;

mod app;
