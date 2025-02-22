use oxidrive_accounts::auth::{AccountEntity, WorkspaceEntity};
use oxidrive_authorization::{Entity, Ref};
use rust_embed::Embed;
use serde::Serialize;

use crate::{
    File, FileId,
    collection::{Collection, CollectionId},
};

#[derive(Embed)]
#[folder = "cedar"]
#[include = "*.cedarschema"]
pub struct FilesAuthSchemas;

#[derive(Embed)]
#[folder = "cedar"]
#[include = "*.cedar"]
pub struct FilesAuthPolicies;

#[derive(Debug, Serialize)]
pub struct FileEntity {
    id: FileId,
    owner: Ref<AccountEntity>,
}

impl Entity for FileEntity {
    const TYPE: &'static str = "File";

    fn id(&self) -> String {
        self.id.to_string()
    }

    fn attrs(&self) -> impl serde::Serialize {
        self
    }

    fn parents(&self) -> Vec<oxidrive_authorization::AnyRef> {
        vec![Ref::<WorkspaceEntity>::new(self.owner.id()).into()]
    }
}

impl From<&File> for FileEntity {
    fn from(file: &File) -> Self {
        Self {
            id: file.id,
            owner: Ref::new(file.owner_id),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CollectionEntity {
    id: CollectionId,
    owner: Ref<AccountEntity>,
}

impl Entity for CollectionEntity {
    const TYPE: &'static str = "Collection";

    fn id(&self) -> String {
        self.id.to_string()
    }

    fn attrs(&self) -> impl serde::Serialize {
        self
    }
}

impl From<&Collection> for CollectionEntity {
    fn from(collection: &Collection) -> Self {
        Self {
            id: collection.id,
            owner: Ref::new(collection.owner_id),
        }
    }
}
