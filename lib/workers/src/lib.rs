use serde::{de::DeserializeOwned, Serialize};
pub use worker::*;

pub mod queue;
mod worker;

pub mod inmemory;
pub mod pg;
pub mod sqlite;

pub trait Process: Send + Sync + 'static {
    type Job: Job;
    type Error: std::error::Error + Send;

    fn process(
        &self,
        job: Self::Job,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
}

pub trait Job: Serialize + DeserializeOwned {
    fn kind() -> std::borrow::Cow<'static, str> {
        std::any::type_name::<Self>().into()
    }
}

#[cfg(test)]
mod tests {
    use std::{convert::Infallible, sync::Arc, time::Duration};

    use assert2::{check, let_assert};
    use queue::{Enqueue, JobQueue};
    use serde::{Deserialize, Serialize};
    use tokio::sync::mpsc;

    use super::*;

    struct TestJobProcessor {
        tx: mpsc::Sender<usize>,
    }

    impl TestJobProcessor {
        fn new(tx: mpsc::Sender<usize>) -> Self {
            Self { tx }
        }
    }

    impl Process for TestJobProcessor {
        type Job = TestJob;

        type Error = Infallible;

        async fn process(&self, job: Self::Job) -> Result<(), Self::Error> {
            self.tx.send(job.a + job.b).await.unwrap();
            Ok(())
        }
    }

    #[derive(Clone, Deserialize, Serialize)]
    struct TestJob {
        a: usize,
        b: usize,
    }

    impl Job for TestJob {}

    async fn queue_job<Q: JobQueue + Enqueue>(queue: Q, timeout: Duration) {
        let queue = Arc::new(queue);

        let (tx, mut rx) = mpsc::channel(1);
        let processor = TestJobProcessor::new(tx);

        let worker = Worker::new(queue.clone(), queue.clone(), processor);
        let jobs = worker.dispatcher();

        worker.start();

        jobs.dispatch(TestJob { a: 2, b: 2 }).await.unwrap();

        let result = tokio::time::timeout(timeout, rx.recv())
            .await
            .expect("timeout while waiting for test job result");

        let_assert!(Some(result) = result);

        check!(result == 4);

        tokio::time::sleep(Duration::from_millis(100)).await;

        let mut pending = queue.pull_batch(&TestJob::kind()).await.unwrap();
        let_assert!(None = pending.next());
    }

    mod inmemory {
        use crate::inmemory::*;

        use super::*;

        #[tokio::test]
        async fn it_queues_a_job() {
            let queue = InMemoryJobQueue::default();
            queue_job(queue, Duration::from_millis(100)).await;
        }
    }

    mod pg {
        use oxidrive_database::migrate::PG_MIGRATOR;
        use sqlx::PgPool;

        use crate::pg::PgJobQueue;

        use super::*;

        #[sqlx::test(migrator = "PG_MIGRATOR")]
        async fn it_queues_a_job(pool: PgPool) {
            let queue = PgJobQueue::new(pool);
            queue_job(queue, Duration::from_secs(1)).await;
        }
    }

    mod sqlite {
        use oxidrive_database::migrate::SQLITE_MIGRATOR;
        use sqlx::SqlitePool;

        use crate::sqlite::SqliteJobQueue;

        use super::*;

        #[sqlx::test(migrator = "SQLITE_MIGRATOR")]
        async fn it_queues_a_job(pool: SqlitePool) {
            let queue = SqliteJobQueue::new(pool);
            queue_job(queue, Duration::from_secs(1)).await;
        }
    }
}
