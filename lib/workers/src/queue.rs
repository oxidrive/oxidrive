use std::{any::Any, ops::Deref};

use async_trait::async_trait;
use oxidrive_domain::{make_error_wrapper, make_uuid_type};

make_uuid_type!(JobId, job_id);

make_error_wrapper!(PullBatchError);
make_error_wrapper!(CommitError);

#[async_trait]
pub trait JobQueue: Send + Sync + 'static {
    async fn pull_batch(&self, kind: &str) -> Result<JobBatch, PullBatchError>;
    async fn commit(&self, batch: JobBatch) -> Result<(), CommitError>;
}

#[async_trait]
impl<T, Q> JobQueue for T
where
    T: Deref<Target = Q> + Send + Sync + 'static,
    Q: JobQueue,
{
    async fn pull_batch(&self, kind: &str) -> Result<JobBatch, PullBatchError> {
        self.deref().pull_batch(kind).await
    }

    async fn commit(&self, batch: JobBatch) -> Result<(), CommitError> {
        self.deref().commit(batch).await
    }
}

make_error_wrapper!(QueueError);

#[async_trait]
pub trait Enqueue: Send + Sync + 'static {
    async fn queue(&self, job: QueuedJob) -> Result<(), QueueError>;
}

#[async_trait]
impl<T, Q> Enqueue for T
where
    T: Deref<Target = Q> + Send + Sync + 'static,
    Q: Enqueue,
{
    async fn queue(&self, job: QueuedJob) -> Result<(), QueueError> {
        self.deref().queue(job).await
    }
}

pub struct JobBatch {
    jobs: std::vec::IntoIter<QueuedJob>,
    successes: Vec<JobId>,
    state: Box<dyn Any + Send>,
}

impl JobBatch {
    pub fn new<I, S>(jobs: I, state: S) -> Self
    where
        I: Iterator<Item = QueuedJob>,
        S: Send + 'static,
    {
        Self {
            jobs: jobs.collect::<Vec<_>>().into_iter(),
            successes: Vec::new(),
            state: Box::new(state),
        }
    }

    pub fn iter(&self) -> &impl Iterator<Item = QueuedJob> {
        &self.jobs
    }

    pub fn iter_mut(&mut self) -> &mut impl Iterator<Item = QueuedJob> {
        &mut self.jobs
    }

    pub fn record(&mut self, successes: Vec<JobId>) {
        self.successes.extend(successes);
    }

    pub fn finish<S: 'static>(self) -> (Vec<JobId>, Option<S>) {
        let state = self.state.downcast().ok().map(|state| *state);

        (self.successes, state)
    }
}

impl Iterator for JobBatch {
    type Item = QueuedJob;

    fn next(&mut self) -> Option<Self::Item> {
        self.jobs.next()
    }
}

#[derive(Clone)]
pub struct QueuedJob {
    pub id: JobId,
    pub kind: std::borrow::Cow<'static, str>,
    pub payload: serde_json::Value,
}
