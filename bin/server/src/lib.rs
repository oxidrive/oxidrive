use std::sync::Arc;

use oxidrive_database::Database;
use oxidrive_files::file::FileContents;
use worker::{job_enqueue, job_queue};

pub mod worker;

#[derive(Clone)]
pub struct ServerModule;

impl app::Module for ServerModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(job_queue);
        c.bind(job_enqueue);
    }
}

#[app::async_trait]
impl app::Hooks for ServerModule {
    async fn after_start(&mut self, c: &app::di::Container) -> eyre::Result<()> {
        let db = c.get::<Database>();
        let contents = c.get::<Arc<dyn FileContents>>();

        tracing::info!("using database {}", db.display_name());
        tracing::info!("using storage {}", contents.display_name());

        Ok(())
    }
}
