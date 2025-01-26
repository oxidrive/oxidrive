use oxidrive_authorization::Entity;
use rust_embed::Embed;
use serde::Serialize;

use crate::account::{Account, AccountId};

#[derive(Embed)]
#[folder = "cedar"]
#[include = "*.cedarschema"]
pub struct AccountsAuthSchemas;

#[derive(Embed)]
#[folder = "cedar"]
#[include = "*.cedar"]
pub struct AccountsAuthPolicies;

#[derive(Debug, Serialize)]
pub struct AccountEntity {
    id: AccountId,
}

impl Entity for AccountEntity {
    const TYPE: &'static str = "Account";

    fn id(&self) -> String {
        self.id.to_string()
    }

    fn attrs(&self) -> impl Serialize {
        self
    }
}

impl From<&Account> for AccountEntity {
    fn from(account: &Account) -> Self {
        Self { id: account.id }
    }
}

#[derive(Debug, Serialize)]
pub struct WorkspaceEntity {
    owner: AccountId,
}

impl Entity for WorkspaceEntity {
    const TYPE: &'static str = "Workspace";

    fn id(&self) -> String {
        self.owner.to_string()
    }

    fn attrs(&self) -> impl Serialize {
        self
    }
}

impl From<&Account> for WorkspaceEntity {
    fn from(account: &Account) -> Self {
        Self { owner: account.id }
    }
}
