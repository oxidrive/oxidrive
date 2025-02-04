use std::sync::Arc;

use oxidrive_workers::{
    queue::{Enqueue, JobQueue},
    Worker,
};

pub use refresh_collection::*;
pub use refresh_collections::*;

mod refresh_collection;
mod refresh_collections;

pub(crate) struct JobsModule;

impl app::Module for JobsModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(RefreshCollectionWorker::new);
        c.bind(
            |queue: Arc<dyn JobQueue>,
             enqueue: Arc<dyn Enqueue>,
             process: RefreshCollectionWorker| { Worker::new(queue, enqueue, process) },
        );
        c.bind(|worker: Worker<RefreshCollectionWorker>| worker.dispatcher());

        c.bind(RefreshCollectionsWorker::new);
        c.bind(
            |queue: Arc<dyn JobQueue>,
             enqueue: Arc<dyn Enqueue>,
             process: RefreshCollectionsWorker| {
                Worker::new(queue, enqueue, process)
            },
        );
        c.bind(|worker: Worker<RefreshCollectionsWorker>| worker.dispatcher());
    }
}

#[app::async_trait]
impl app::Hooks for JobsModule {
    async fn after_start(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        c.get::<Worker<RefreshCollectionWorker>>().clone().start();
        c.get::<Worker<RefreshCollectionsWorker>>().clone().start();

        Ok(())
    }
}
