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

    pub fn start(self, ctx: app::context::Context) {
        let run = async move {
            let mut interval = tokio::time::interval(self.every);

            loop {
                interval.tick().await;
                let job = (self.make_job)();
                tracing::trace!(job = %J::kind(), every = ?self.every, "scheduling job");
                if let Err(err) = self.dispatch.dispatch(job).await {
                    tracing::error!(error = %err, error.details = ?err, "job scheduling failed");
                }
            }
        };

        tokio::spawn(async move {
            tracing::info!(job = %J::kind(), "starting job scheduler");
            tokio::select! {
                _ = run => {},
                _ = ctx.cancelled() => {},
            };
            tracing::info!(job = %J::kind(), "stopping job scheduler");
        });
    }
}
