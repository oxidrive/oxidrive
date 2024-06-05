mod button;
mod input;
mod loading;
mod logo;
mod navbar;
mod pane;
mod title;

pub use button::*;
pub use input::*;
pub use loading::*;
pub use logo::*;
pub use navbar::*;
pub use pane::*;
pub use title::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    Medium,
    Large,
}

impl Size {
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::Medium => "md",
            Self::Large => "lg",
        }
    }
}
