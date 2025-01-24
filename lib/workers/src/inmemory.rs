use std::{collections::VecDeque, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::queue::{
    CommitError, Enqueue, JobBatch, JobQueue, PullBatchError, QueueError, QueuedJob,
};

#[derive(Clone, Default)]
pub struct InMemoryJobQueue {
    queue: Arc<RwLock<VecDeque<QueuedJob>>>,
}

#[async_trait]
impl JobQueue for InMemoryJobQueue {
    async fn pull_batch(&self, kind: &str) -> Result<JobBatch, PullBatchError> {
        let queue = self.queue.read().await;
        let batch = queue.iter().filter(|job| job.kind == kind).cloned();
        Ok(JobBatch::new(batch, ()))
    }

    async fn commit(&self, batch: JobBatch) -> Result<(), CommitError> {
        let mut queue = self.queue.write().await;
        let (successes, _) = batch.finish::<()>();
        queue.retain(|job| !successes.contains(&job.id));
        Ok(())
    }
}

#[async_trait]
impl Enqueue for InMemoryJobQueue {
    async fn queue(&self, job: QueuedJob) -> Result<(), QueueError> {
        let mut queue = self.queue.write().await;
        queue.push_back(job);
        Ok(())
    }
}
