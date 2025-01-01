use async_trait::async_trait;

use crate::di::Container;

#[async_trait]
#[allow(unused_variables)] // It's annoying when you implement them
pub trait Hooks: Send + 'static {
    async fn before_start(&mut self, c: &Container) -> eyre::Result<()> {
        Ok(())
    }

    async fn on_shutdown(&mut self, c: &Container) -> eyre::Result<()> {
        Ok(())
    }
}
