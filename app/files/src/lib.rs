use std::{path::PathBuf, sync::Arc};

use collection::CollectionsModule;
use file::{FileContents, FileMetadata, FsFileContents, PgFileMetadata, SqliteFileMetadata};
use oxidrive_database::Database;
use serde::Deserialize;

pub use file::{ContentStreamError, File, FileId};
pub use service::*;
pub use tag::Tag;

pub mod collection;
mod content_type;
pub mod file;
mod service;
pub mod tag;

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

fn contents(cfg: Config) -> Arc<dyn FileContents> {
    match cfg {
        Config::FileSystem { root_folder_path } => Arc::new(FsFileContents::new(root_folder_path)),
    }
}
