use sqlx::migrate::Migrator;

use crate::Database;

pub static PG_MIGRATOR: Migrator = sqlx::migrate!("../../migrations/postgres");
pub static SQLITE_MIGRATOR: Migrator = sqlx::migrate!("../../migrations/sqlite");

pub async fn migrate(_ctx: app::context::Context, db: &Database) -> app::eyre::Result<()> {
    tracing::debug!("running {} migrations", db.name());

    match db {
        Database::Pg(pool) => PG_MIGRATOR.run(pool).await?,
        Database::Sqlite(pool) => SQLITE_MIGRATOR.run(pool).await?,
    }

    tracing::debug!("database migrated successfully");
    Ok(())
}
