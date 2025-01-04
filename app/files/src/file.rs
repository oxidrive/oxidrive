use oxidrive_auth::account::AccountId;
use oxidrive_domain::make_uuid_type;

make_uuid_type!(FileId, file_id);

mod content;
mod store;

pub use content::*;
pub use store::*;

#[derive(Clone)]
pub struct File {
    pub id: FileId,
    pub owner_id: AccountId,
    pub name: String,
    pub content_type: String,
}

impl File {
    pub fn create(
        owner_id: AccountId,
        name: impl Into<String>,
        content_type: impl Into<String>,
    ) -> Self {
        Self {
            id: FileId::new(),
            owner_id,
            name: name.into(),
            content_type: content_type.into(),
        }
    }
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use bytes::Bytes;
    use fake::Fake;
    use futures::Stream;
    use oxidrive_auth::account::{fixtures::account, Account};
    use rstest::fixture;

    use super::*;

    #[fixture]
    pub fn file(account: Account) -> File {
        File {
            id: FileId::new(),
            owner_id: account.id,
            name: fake::faker::filesystem::en::FileName().fake::<String>(),
            content_type: fake::faker::filesystem::en::MimeType().fake::<String>(),
        }
    }

    #[allow(dead_code)] // this is used in tests and such, but clippy doesn't seem to detect it
    pub fn content(
        content: impl Into<Bytes>,
    ) -> impl Stream<Item = Result<Bytes, ContentStreamError>> {
        futures::stream::once(async move { Ok(content.into()) })
    }
}

#[cfg(test)]
mod tests {}
