use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, sqlite::SqlitePoolOptions};
use url::Url;

pub mod migrate;

pub use migrate::migrate;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    url: Url,
    #[serde(default = "default_max_conn")]
    max_connections: u32,
}

fn default_max_conn() -> u32 {
    10
}

#[derive(Clone)]
pub enum Database {
    Sqlite(sqlx::SqlitePool),
    Pg(sqlx::PgPool),
}

impl Database {
    pub fn name(&self) -> &'static str {
        match self {
            Database::Sqlite(_) => "sqlite",
            Database::Pg(_) => "postgres",
        }
    }
}

#[derive(Clone)]
pub struct DatabaseModule;

app::provides!(DatabaseModule, Database);

impl app::Module for DatabaseModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(database);
    }
}

fn database(cfg: Config) -> Database {
    match cfg.url.scheme() {
        "postgres" | "postgresql" => {
            let pool = PgPoolOptions::new()
                .max_connections(cfg.max_connections)
                .connect_lazy(cfg.url.as_str())
                .expect("failed to connect to PostgreSQL database");

            Database::Pg(pool)
        }
        "sqlite" => {
            let pool = SqlitePoolOptions::new()
                .max_connections(cfg.max_connections)
                .connect_lazy(cfg.url.as_str())
                .expect("failed to open SQLite database");

            Database::Sqlite(pool)
        }
        scheme => panic!(
            "invalid database URL {}: unsupported database '{scheme}'",
            cfg.url
        ),
    }
}

#[app::async_trait]
impl app::Hooks for DatabaseModule {
    async fn before_start(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        let db = c.get::<Database>();

        migrate(db).await?;

        Ok(())
    }

    async fn on_shutdown(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        let db = c.get::<Database>();

        match db {
            Database::Sqlite(pool) => pool.close().await,
            Database::Pg(pool) => pool.close().await,
        }

        Ok(())
    }
}
