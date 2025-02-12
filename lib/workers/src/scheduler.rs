use std::time::Duration;

use crate::{Dispatch, Job};

pub struct Scheduler<J, M> {
    every: Duration,
    dispatch: Dispatch<J>,
    make_job: M,
}

impl<J, M> Scheduler<J, M>
where
    J: Job + Send + Sync + 'static,
    M: Fn() -> J + Send + Sync + 'static,
{
    pub fn new(every: Duration, dispatch: Dispatch<J>, make_job: M) -> Self {
        Self {
            every,
            dispatch,
            make_job,
        }
    }

    pub fn start(self) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(self.every);

            loop {
                interval.tick().await;
                let job = (self.make_job)();
                tracing::trace!(job.kind = %J::kind(), every = ?self.every, "scheduling job");
                if let Err(err) = self.dispatch.dispatch(job).await {
                    tracing::error!(error = %err, error.details = ?err, "job scheduling failed");
                }
            }
        });
    }
}
