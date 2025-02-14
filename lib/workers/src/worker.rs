use std::{marker::PhantomData, sync::Arc, time::Duration};

use serde::Serialize;
use tokio::task::JoinError;

use crate::{
    queue::{CommitError, Enqueue, JobId, JobQueue, PullBatchError, QueueError, QueuedJob},
    Job, Process,
};

pub struct Worker<P> {
    queue: Arc<dyn JobQueue>,
    enqueuer: Arc<dyn Enqueue>,
    processor: Arc<P>,
    poll_timeout: Duration,
}

impl<P> Worker<P>
where
    P: Process,
{
    pub fn new(queue: Arc<dyn JobQueue>, enqueuer: Arc<dyn Enqueue>, processor: P) -> Self {
        Self {
            queue,
            enqueuer,
            processor: Arc::new(processor),
            poll_timeout: Duration::from_secs(1),
        }
    }
}
impl<P> Worker<P>
where
    P: Process,
{
    pub fn with_poll_timeout(mut self, timeout: Duration) -> Self {
        self.poll_timeout = timeout;
        self
    }

    pub fn dispatcher(&self) -> Dispatch<P::Job> {
        let queue = self.enqueuer.clone();

        Dispatch {
            queue,
            _jobs: PhantomData,
        }
    }

    pub fn start(self, ctx: app::context::Context) {
        tokio::spawn(async move {
            let worker = std::any::type_name::<P>();
            let job = P::Job::kind();

            tracing::info!(worker, %job, "starting background worker");

            let run = async move {
                loop {
                    if let Err(err) = self.run_batch().await {
                        tracing::error!(error = %err, error.details = ?err, "job processing failed");
                    }

                    tokio::time::sleep(self.poll_timeout).await;
                }
            };

            tokio::select! {
                _ = run => {},
                _ = ctx.cancelled() => {},
            }

            tracing::info!(worker, %job, "stopping background worker");
        });
    }

    async fn run_batch(&self) -> Result<(), RunError<P::Error>> {
        let mut batch = self
            .queue
            .pull_batch(&P::Job::kind())
            .await
            .map_err(RunError::PullBatchFailed)?;

        let mut results = if let (_, Some(size_hint)) = batch.size_hint() {
            Vec::with_capacity(size_hint)
        } else {
            Vec::new()
        };

        for job in batch.by_ref() {
            let processor = self.processor.clone();
            let handle = tokio::spawn(async move {
                let id = job.id;
                let job = serde_json::from_value(job.payload)?;
                processor
                    .process(job)
                    .await
                    .map_err(ProcessError::Process)?;
                Result::<_, ProcessError<P::Error>>::Ok(id)
            });
            results.push(handle);
        }

        let results = futures::future::try_join_all(results).await?;

        if results.is_empty() {
            return Ok(());
        }

        let (successes, errors): (Vec<_>, Vec<_>) =
            results.into_iter().partition(|res| res.is_ok());

        let successes = successes
            .into_iter()
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        let errors = errors
            .into_iter()
            .map(Result::unwrap_err)
            .collect::<Vec<_>>();

        batch.record(successes);

        self.queue
            .commit(batch)
            .await
            .map_err(RunError::CommitFailed)?;

        if !errors.is_empty() {
            return Err(RunError::BatchErrors(errors));
        }

        Ok(())
    }
}

impl<P> Clone for Worker<P> {
    fn clone(&self) -> Self {
        Self {
            queue: self.queue.clone(),
            enqueuer: self.enqueuer.clone(),
            processor: self.processor.clone(),
            poll_timeout: self.poll_timeout,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RunError<P: std::error::Error> {
    #[error("failed to pull job batch from queue: {0}")]
    PullBatchFailed(#[from] PullBatchError),
    #[error("failed to wait for batch of jobs to complete")]
    JoinBatchFailed(#[from] JoinError),
    #[error("some jobs failed")]
    BatchErrors(Vec<ProcessError<P>>),
    #[error("failed to commit successful jobs: {0}")]
    CommitFailed(#[from] CommitError),
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessError<P: std::error::Error> {
    #[error("job processing failed: {0}")]
    Process(#[source] P),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub struct Dispatch<J> {
    queue: Arc<dyn Enqueue>,
    _jobs: PhantomData<J>,
}

impl<J> Clone for Dispatch<J> {
    fn clone(&self) -> Self {
        Self {
            queue: self.queue.clone(),
            _jobs: self._jobs,
        }
    }
}

impl<J: Job + Serialize> Dispatch<J> {
    pub async fn dispatch(&self, job: J) -> Result<(), DispatchError> {
        let payload = serde_json::to_value(job)?;

        let id = JobId::new();

        self.queue
            .queue(QueuedJob {
                id,
                kind: J::kind(),
                payload,
            })
            .await?;

        tracing::trace!(job.id = %id, job.kind = %J::kind(), "dispatched job");
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("failed to serialize job to JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("failed to push job to job queue: {0}")]
    QueueFailed(#[from] QueueError),
}
