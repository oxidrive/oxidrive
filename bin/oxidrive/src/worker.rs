use std::sync::Arc;

use oxidrive_database::Database;
use oxidrive_workers::{
    pg::PgJobQueue,
    queue::{Enqueue, JobQueue},
    sqlite::SqliteJobQueue,
};

pub fn job_queue(database: Database) -> Arc<dyn JobQueue> {
    match database {
        Database::Sqlite(pool) => Arc::new(SqliteJobQueue::new(pool)),
        Database::Pg(pool) => Arc::new(PgJobQueue::new(pool)),
    }
}

pub fn job_enqueue(database: Database) -> Arc<dyn Enqueue> {
    match database {
        Database::Sqlite(pool) => Arc::new(SqliteJobQueue::new(pool)),
        Database::Pg(pool) => Arc::new(PgJobQueue::new(pool)),
    }
}
