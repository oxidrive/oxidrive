use std::sync::Arc;

use oxidrive_workers::{
    queue::{Enqueue, JobQueue},
    scheduler::Scheduler,
    Job, Process, Worker,
};
use serde::{Deserialize, Serialize};

use super::{DeleteExpiredError, Sessions};

#[derive(Default, Serialize, Deserialize)]
struct CleanupExpiredSessions;

impl Job for CleanupExpiredSessions {}

#[derive(Clone)]
struct CleanupExpiredSessionsWorker {
    sessions: Sessions,
}

impl CleanupExpiredSessionsWorker {
    pub fn new(sessions: Sessions) -> Self {
        Self { sessions }
    }
}

impl Process for CleanupExpiredSessionsWorker {
    type Job = CleanupExpiredSessions;

    type Error = DeleteExpiredError;

    async fn process(&self, _: Self::Job) -> Result<(), Self::Error> {
        self.sessions.delete_expired().await?;
        Ok(())
    }
}

pub(crate) struct JobsModule;

impl app::Module for JobsModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(CleanupExpiredSessionsWorker::new);
        c.bind(
            |queue: Arc<dyn JobQueue>,
             enqueue: Arc<dyn Enqueue>,
             process: CleanupExpiredSessionsWorker| {
                Worker::new(queue, enqueue, process)
            },
        );
    }
}

#[app::async_trait]
impl app::Hooks for JobsModule {
    async fn after_start(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        let worker = c.get::<Worker<CleanupExpiredSessionsWorker>>();
        let dispatch = worker.dispatcher();

        worker.clone().start();

        // TODO: make it configurable
        let every = time::Duration::hours(1).try_into().unwrap();

        Scheduler::new(every, dispatch, CleanupExpiredSessions::default).start();
        Ok(())
    }
}
