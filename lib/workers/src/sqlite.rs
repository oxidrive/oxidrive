use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite, Transaction, types::Json};

use crate::queue::{
    CommitError, Enqueue, JobBatch, JobQueue, PullBatchError, QueueError, QueuedJob,
};

#[derive(Clone)]
pub struct SqliteJobQueue {
    pool: sqlx::SqlitePool,
}

impl SqliteJobQueue {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobQueue for SqliteJobQueue {
    async fn pull_batch(&self, kind: &str) -> Result<JobBatch, PullBatchError> {
        let mut tx = self.pool.begin().await.map_err(PullBatchError::wrap)?;

        let batch: Vec<SqliteJob> =
            sqlx::query_as("select id, kind, body from jobs where kind = ? order by id limit 10")
                .bind(kind)
                .fetch_all(&mut *tx)
                .await
                .map_err(PullBatchError::wrap)?;

        let batch = batch.into_iter().map(Into::into);
        Ok(JobBatch::new(batch, tx))
    }

    async fn commit(&self, batch: JobBatch) -> Result<(), CommitError> {
        let (successes, tx) = batch.finish::<Transaction<'static, Sqlite>>();
        let mut tx = tx.unwrap();

        let ids = successes.into_iter().map(|id| id.to_string());

        let mut qb = QueryBuilder::new("delete from jobs where id in (");

        let mut values = qb.separated(", ");
        for id in ids {
            values.push_bind(id);
        }

        values.push_unseparated(")");

        qb.build()
            .execute(&mut *tx)
            .await
            .map_err(CommitError::wrap)?;

        tx.commit().await.map_err(CommitError::wrap)?;

        Ok(())
    }
}

#[async_trait]
impl Enqueue for SqliteJobQueue {
    async fn queue(&self, job: QueuedJob) -> Result<(), QueueError> {
        sqlx::query("insert into jobs (id, kind, body) values (?, ?, ?)")
            .bind(job.id.to_string())
            .bind(job.kind)
            .bind(Json(job.payload))
            .execute(&self.pool)
            .await
            .map_err(QueueError::wrap)?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct SqliteJob {
    id: String,
    kind: String,
    body: Json<serde_json::Value>,
}

impl From<SqliteJob> for QueuedJob {
    fn from(SqliteJob { id, kind, body }: SqliteJob) -> Self {
        QueuedJob {
            id: id.parse().unwrap(),
            kind: kind.into(),
            payload: body.0,
        }
    }
}
