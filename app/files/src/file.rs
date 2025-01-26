use std::collections::HashMap;
use std::str::FromStr;

use oxidrive_accounts::account::AccountId;
use oxidrive_domain::make_uuid_type;

mod content;
pub(crate) mod store;

pub use content::*;
pub use store::*;

use crate::tag;
use crate::tag::reserved::SIZE;
use crate::tag::Tag;

pub type Tags = HashMap<String, Tag>;

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

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
        self.add_tag(Tag::full(SIZE, size.to_string()));
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

    pub fn set_tags<I>(&mut self, tags: I)
    where
        I: IntoIterator<Item = Tag>,
    {
        self.tags = Self::default_tags(self);
        let tags = tags.into_iter().filter(Tag::is_public);

        self.add_tags(tags);
    }

    pub fn add_tags<I>(&mut self, tags: I)
    where
        I: IntoIterator<Item = Tag>,
    {
        self.tags
            .extend(tags.into_iter().map(|tag| (tag.key.clone(), tag)));
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag.key.clone(), tag);
    }

    fn default_tags(file: &File) -> Tags {
        let mut tags = HashMap::from_iter(
            [
                tag!("{}:{}", tag::reserved::NAME, file.name),
                tag!("{}:{}", tag::reserved::CONTENT_TYPE, file.content_type),
                tag!("{}:{}", tag::reserved::SIZE, file.size),
            ]
            .into_iter()
            .map(|tag| (tag.key.clone(), tag)),
        );

        if let Some(ext) = std::path::PathBuf::from_str(&file.name)
            .ok()
            .as_ref()
            .and_then(|p| p.extension())
            .and_then(|ext| ext.to_str())
        {
            let tag = tag!("{}:{}", tag::reserved::FILE_EXT, ext);
            tags.insert(tag.key.clone(), tag);
        }

        tags
    }
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use bytes::Bytes;
    use fake::Fake;
    use futures::Stream;
    use oxidrive_accounts::account::{fixtures::account, Account};
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
    use oxidrive_accounts::account::{fixtures::account, Account};
    use rstest::rstest;

    use crate::tag::reserved::*;

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

    #[rstest]
    fn it_updates_the_tags_without_overriding_the_default_ones(mut file: File) {
        let name = file.tags.get(NAME).cloned().unwrap();
        let content_type = file.tags.get(CONTENT_TYPE).cloned().unwrap();
        let size = file.tags.get(SIZE).cloned().unwrap();

        file.set_tags([
            tag!("name:different"),
            tag!("content_type:changed"),
            tag!("size:0"),
            tag!("added"),
        ]);

        check!(file.tags.get(NAME) == Some(&name));
        check!(file.tags.get(CONTENT_TYPE) == Some(&content_type));
        check!(file.tags.get(SIZE) == Some(&size));
        check!(file.tags.get("added") == Some(&tag!("added")));
    }
}
