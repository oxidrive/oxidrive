use async_trait::async_trait;

use crate::{context::Context, di::Container};

#[async_trait]
#[allow(unused_variables)] // It's annoying when you implement them
pub trait Hooks: Send + 'static {
    async fn before_start(&mut self, ctx: Context, c: &Container) -> eyre::Result<()> {
        Ok(())
    }

    async fn after_start(&mut self, ctx: Context, c: &Container) -> eyre::Result<()> {
        Ok(())
    }

    async fn on_shutdown(&mut self, ctx: Context, c: &Container) -> eyre::Result<()> {
        Ok(())
    }
}
