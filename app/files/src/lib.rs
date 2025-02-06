use std::{path::PathBuf, sync::Arc};

use collection::CollectionsModule;
use file::{FileMetadata, FileStorage, PgFileMetadata, SqliteFileMetadata};
use oxidrive_database::Database;
use serde::Deserialize;

pub use file::{File, FileId};
pub use service::*;
pub use tag::Tag;

pub mod auth;
pub mod collection;
mod content_type;
pub mod file;
mod service;
pub mod tag;

#[derive(Clone)]
pub struct FilesModule;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "provider")]
pub enum Config {
    #[serde(alias = "fs")]
    FileSystem { root_folder_path: PathBuf },
}

impl app::Module for FilesModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(metadata);
        c.bind(contents);
        c.mount(CollectionsModule);
        c.bind(Files::new);
    }
}

fn metadata(database: Database) -> Arc<dyn FileMetadata> {
    match database {
        Database::Sqlite(pool) => Arc::new(SqliteFileMetadata::new(pool)),
        Database::Pg(pool) => Arc::new(PgFileMetadata::new(pool)),
    }
}

fn contents(cfg: Config) -> FileStorage {
    match cfg {
        Config::FileSystem { root_folder_path } => FileStorage::file_system(root_folder_path),
    }
}

#[app::async_trait]
impl app::Hooks for FilesModule {
    async fn before_start(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        CollectionsModule.before_start(c).await?;
        Ok(())
    }

    async fn after_start(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        CollectionsModule.after_start(c).await?;
        Ok(())
    }

    async fn on_shutdown(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        CollectionsModule.on_shutdown(c).await?;
        Ok(())
    }
}
