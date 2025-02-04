use std::{collections::HashSet, sync::Arc};

use jobs::JobsModule;
use oxidrive_accounts::account::AccountId;
use oxidrive_database::Database;
use oxidrive_domain::make_uuid_type;
use oxidrive_search::Filter;

pub use service::*;
pub use store::*;

use crate::FileId;

pub mod jobs;
mod service;
mod store;

make_uuid_type!(CollectionId, collection_id);

#[derive(Debug, Clone)]
pub struct Collection {
    pub id: CollectionId,
    pub name: String,
    pub owner_id: AccountId,
    filter: Filter,
    files: HashSet<FileId>,
}

impl Collection {
    pub fn new(owner_id: AccountId, name: impl Into<String>, filter: Filter) -> Self {
        Collection {
            id: CollectionId::new(),
            owner_id,
            name: name.into(),
            filter,
            files: Default::default(),
        }
    }

    pub fn files(&self) -> impl Iterator<Item = FileId> + use<'_> {
        self.files.iter().copied()
    }

    fn add<I>(&mut self, files: I)
    where
        I: IntoIterator<Item = FileId>,
    {
        self.files.extend(files);
    }

    pub fn filter(&self) -> &Filter {
        &self.filter
    }
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use fake::Fake;
    use oxidrive_accounts::account::{fixtures::account, Account};
    use rstest::fixture;

    use super::*;

    #[fixture]
    pub fn collection(account: Account) -> Collection {
        let name = fake::faker::lorem::en::Sentence(1..5).fake::<String>();
        Collection::new(account.id, name, Filter::All)
    }
}

#[derive(Copy, Clone)]
pub struct CollectionsModule;

impl app::Module for CollectionsModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(store);
        c.mount(JobsModule);
        c.bind(Collections::new);
    }
}

fn store(database: Database) -> Arc<dyn CollectionStore> {
    match database {
        Database::Sqlite(pool) => Arc::new(SqliteCollectionStore::new(pool)),
        Database::Pg(pool) => Arc::new(PgCollectionStore::new(pool)),
    }
}

#[app::async_trait]
impl app::Hooks for CollectionsModule {
    async fn before_start(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        JobsModule.before_start(c).await?;
        Ok(())
    }

    async fn after_start(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        JobsModule.after_start(c).await?;
        Ok(())
    }

    async fn on_shutdown(&mut self, c: &app::di::Container) -> app::eyre::Result<()> {
        JobsModule.on_shutdown(c).await?;
        Ok(())
    }
}
