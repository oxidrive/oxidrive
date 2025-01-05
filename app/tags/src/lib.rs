use std::sync::Arc;

use oxidrive_database::Database;
use oxidrive_files::{file::FileId, File};
pub use tag::*;

mod tag;

#[derive(Clone)]
pub struct Tags {
    store: Arc<dyn TagIndex>,
}

impl Tags {
    fn new(store: Arc<dyn TagIndex>) -> Self {
        Self { store }
    }

    pub async fn index<I>(&self, file: &File, tags: I) -> Result<(), IndexTagError>
    where
        I: IntoIterator<Item = Tag>,
    {
        self.store.index(file, tags.into_iter().collect()).await
    }

    pub async fn for_file(&self, file_id: FileId) -> Result<Vec<Tag>, ForFileError> {
        self.store.for_file(file_id).await
    }
}

pub struct TagsModule;

impl app::Module for TagsModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(index);
        c.bind(Tags::new);
    }
}

fn index(database: Database) -> Arc<dyn TagIndex> {
    match database {
        Database::Sqlite(pool) => Arc::new(SqliteTagIndex::new(pool)),
        Database::Pg(pool) => Arc::new(PgTagIndex::new(pool)),
    }
}
