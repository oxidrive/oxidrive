use std::collections::HashSet;

use oxidrive_auth::account::AccountId;
use oxidrive_domain::make_uuid_type;

mod content;
mod store;

pub use content::*;
pub use store::*;

use crate::tag;
use crate::tag::Tag;

pub type Tags = HashSet<Tag>;

make_uuid_type!(FileId, file_id);

#[derive(Clone, Debug)]
pub struct File {
    pub id: FileId,
    pub owner_id: AccountId,
    pub name: String,
    pub content_type: String,
    pub size: usize,
    pub tags: Tags,
}

impl File {
    pub fn new(
        owner_id: AccountId,
        name: impl Into<String>,
        content_type: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let content_type = content_type.into();

        let mut this = Self {
            id: FileId::new(),
            owner_id,
            name,
            content_type,
            size: 0,
            tags: Default::default(),
        };

        this.tags = Self::default_tags(&this);

        this
    }

    pub fn with_tags<I>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = Tag>,
    {
        self.add_tags(tags);
        self
    }

    pub fn tagged(mut self, tag: Tag) -> Self {
        self.add_tag(tag);
        self
    }

    pub fn add_tags<I>(&mut self, tags: I)
    where
        I: IntoIterator<Item = Tag>,
    {
        self.tags.extend(tags);
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }

    fn default_tags(file: &File) -> Tags {
        HashSet::from([
            tag!("{}:{}", tag::reserved::NAME, file.name),
            tag!("{}:{}", tag::reserved::CONTENT_TYPE, file.content_type),
        ])
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
        File::new(
            account.id,
            fake::faker::filesystem::en::FileName().fake::<String>(),
            fake::faker::filesystem::en::MimeType().fake::<String>(),
        )
    }

    #[allow(dead_code)] // this is used in tests and such, but clippy doesn't seem to detect it
    pub fn content(
        content: impl Into<Bytes>,
    ) -> impl Stream<Item = Result<Bytes, ContentStreamError>> {
        futures::stream::once(async move { Ok(content.into()) })
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use fake::Fake;
    use fixtures::file;
    use oxidrive_auth::account::{fixtures::account, Account};
    use rstest::rstest;

    use crate::tag;

    use super::*;

    #[rstest]
    fn it_creates_a_new_file_with_default_tags(account: Account) {
        let name = fake::faker::filesystem::en::FileName().fake::<String>();
        let content_type = fake::faker::filesystem::en::MimeType().fake::<String>();

        let file = File::new(account.id, name.clone(), content_type.clone());

        let default_tags = File::default_tags(&file);

        check!(file.tags == default_tags);
        check!(file.size == 0);
    }

    #[rstest]
    fn it_does_not_add_duplicated_tags(mut file: File) {
        let default_tags = File::default_tags(&file);

        file.add_tags([tag!("test"), tag!("test")]);

        check!(file.tags.len() == default_tags.len() + 1);
    }
}
