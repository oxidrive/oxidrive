use oxidrive_accounts::auth::{AccountsAuthPolicies, AccountsAuthSchemas};
use oxidrive_authorization::{
    AuthorizationModule,
    cedar::{policies::CompoundPolicyLoader, schema::CompoundSchemaLoader},
};
use oxidrive_database::Database;
use oxidrive_files::{
    auth::{FilesAuthPolicies, FilesAuthSchemas},
    file::FileStorage,
};
use worker::{job_enqueue, job_queue};

pub mod command;
pub mod worker;

#[derive(Clone)]
pub struct ServerModule;

impl app::Module for ServerModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(job_queue);
        c.bind(job_enqueue);
    }
}

#[app::async_trait]
impl app::Hooks for ServerModule {
    async fn after_start(
        &mut self,
        _ctx: app::context::Context,
        c: &app::di::Container,
    ) -> eyre::Result<()> {
        let db = c.get::<Database>();
        let contents = c.get::<FileStorage>();

        tracing::info!("using database {}", db.display_name());
        tracing::info!("using storage {}", contents.display_name());

        Ok(())
    }
}

pub struct PoliciesModule;

impl app::Module for PoliciesModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.add(
            CompoundSchemaLoader::default()
                .load(AccountsAuthSchemas)
                .load(FilesAuthSchemas),
        );

        c.add(
            CompoundPolicyLoader::default()
                .load(AccountsAuthPolicies)
                .load(FilesAuthPolicies),
        );

        c.mount(AuthorizationModule);
    }
}
