use async_trait::async_trait;
use sqlx::{types::Json, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

use crate::queue::{
    CommitError, Enqueue, JobBatch, JobQueue, PullBatchError, QueueError, QueuedJob,
};

#[derive(Clone)]
pub struct PgJobQueue {
    pool: sqlx::PgPool,
}

impl PgJobQueue {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobQueue for PgJobQueue {
    async fn pull_batch(&self, kind: &str) -> Result<JobBatch, PullBatchError> {
        let mut tx = self.pool.begin().await.map_err(PullBatchError::wrap)?;

        let batch: Vec<PgJob> = sqlx::query_as(
            r#"
select
  id,
  kind,
  body
from jobs
where kind = $1
order by id
for update skip locked
limit 10
"#,
        )
        .bind(kind)
        .fetch_all(&mut *tx)
        .await
        .map_err(PullBatchError::wrap)?;

        let batch = batch.into_iter().map(Into::into);
        Ok(JobBatch::new(batch, tx))
    }

    async fn commit(&self, batch: JobBatch) -> Result<(), CommitError> {
        let (successes, tx) = batch.finish::<Transaction<'static, Postgres>>();
        let mut tx = tx.unwrap();

        let ids = successes.into_iter().map(|id| id.as_uuid());

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
impl Enqueue for PgJobQueue {
    async fn queue(&self, job: QueuedJob) -> Result<(), QueueError> {
        sqlx::query("insert into jobs (id, kind, body) values ($1, $2, $3)")
            .bind(job.id.as_uuid())
            .bind(job.kind)
            .bind(Json(job.payload))
            .execute(&self.pool)
            .await
            .map_err(QueueError::wrap)?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PgJob {
    id: Uuid,
    kind: String,
    body: Json<serde_json::Value>,
}

impl From<PgJob> for QueuedJob {
    fn from(PgJob { id, kind, body }: PgJob) -> Self {
        QueuedJob {
            id: id.into(),
            kind: kind.into(),
            payload: body.0,
        }
    }
}
