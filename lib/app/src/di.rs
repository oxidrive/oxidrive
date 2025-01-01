mod container;
mod provider;

pub use container::*;
pub use provider::*;

pub trait Module {
    fn mount(self: Box<Self>, c: &mut Context);
}
