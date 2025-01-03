use std::{borrow::Cow, fmt::Debug, string::FromUtf8Error};

use futures::{future::BoxFuture, FutureExt};
use rust_embed::Embed;
use sqlx::migrate::{Migration, MigrationSource, MigrationType, Migrator};

use crate::Database;

pub async fn migrate(db: &Database) -> app::eyre::Result<()> {
    tracing::debug!("running {} migrations", db.name());

    let migrator = Migrator::new(EmbeddedMigrations::new(db)).await?;

    match db {
        Database::Sqlite(pool) => migrator.run(pool).await?,
        Database::Pg(pool) => migrator.run(pool).await?,
    }

    tracing::debug!("database migrated successfully");
    Ok(())
}

#[derive(Embed)]
#[folder = "../../migrations/sqlite"]
struct SqliteMigrations;

#[derive(Embed)]
#[folder = "../../migrations/postgres"]
struct PgMigrations;

struct EmbeddedMigrations<'a> {
    db: &'a Database,
}

impl<'a> EmbeddedMigrations<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }
}

impl<'s> MigrationSource<'s> for EmbeddedMigrations<'_> {
    fn resolve(self) -> BoxFuture<'s, Result<Vec<Migration>, sqlx::error::BoxDynError>> {
        let migrations = match self.db {
            Database::Sqlite(_) => resolve::<SqliteMigrations>(),
            Database::Pg(_) => resolve::<PgMigrations>(),
        }
        .map_err(sqlx::error::BoxDynError::from);

        futures::future::ready(migrations).boxed()
    }
}

impl Debug for EmbeddedMigrations<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EmbeddedMigrations")
            .field(&match self.db {
                Database::Sqlite(_) => "Sqlite",
                Database::Pg(_) => "Pg",
            })
            .finish()
    }
}

fn resolve<S: Embed>() -> Result<Vec<Migration>, ResolveError> {
    let mut migrations = Vec::new();

    for file_name in S::iter() {
        let file = S::get(&file_name).unwrap();

        // ported from sqlx-core/src/migrate/source.rs

        let parts = file_name.splitn(2, '_').collect::<Vec<_>>();

        if parts.len() != 2 || !parts[1].ends_with(".sql") {
            // not of the format: <VERSION>_<DESCRIPTION>.<REVERSIBLE_DIRECTION>.sql; ignore
            continue;
        }

        let version: i64 = parts[0]
            .parse()
            .map_err(|_e| ResolveError::InvalidName(file_name.clone()))?;

        let migration_type = MigrationType::from_filename(parts[1]);

        // remove the `.sql` and replace `_` with ` `
        let description = parts[1]
            .trim_end_matches(migration_type.suffix())
            .replace('_', " ")
            .to_owned();

        let sql = String::from_utf8(file.data.into())?;

        // opt-out of migration transaction
        let no_tx = sql.starts_with("-- no-transaction");

        migrations.push(Migration::new(
            version,
            description.into(),
            migration_type,
            sql.into(),
            no_tx,
        ));
    }

    Ok(migrations)
}

#[derive(Debug, thiserror::Error)]
enum ResolveError {
    #[error(
        "error parsing migration filename {0}; expected integer version prefix (e.g. `01_foo.sql`)"
    )]
    InvalidName(Cow<'static, str>),

    #[error("invalid migration file content: {0}")]
    InvalidContent(#[from] FromUtf8Error),
}
