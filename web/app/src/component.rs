mod button;
mod input;
mod logo;
mod title;

pub use button::*;
pub use input::*;
pub use logo::*;
pub use title::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    XS,
    Small,
    Medium,
    Large,
    XL,
}

impl Size {
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Self::XS => "xs",
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
            Self::XL => "xl",
        }
    }
}
