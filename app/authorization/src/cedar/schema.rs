use cedar_policy::{Schema, SchemaError, SchemaFragment};
use rust_embed::Embed;

pub trait SchemaLoader {
    fn load(&self) -> impl Iterator<Item = SchemaFragment>;
}

impl<T> SchemaLoader for T
where
    T: Embed,
{
    fn load(&self) -> impl Iterator<Item = SchemaFragment> {
        Self::iter().map(|path| {
            let file = Self::get(&path).unwrap();
            let src = String::from_utf8_lossy(&file.data);

            let (fragment, warnings) =
                SchemaFragment::from_cedarschema_str(&src).unwrap_or_else(|err| {
                    panic!("failed to parse Cedar schema {path}: {err} ({err:?})")
                });

            for warning in warnings {
                tracing::warn!(schema = %path, "{warning}");
            }

            fragment
        })
    }
}

#[derive(Clone, Default)]
pub struct CompoundSchemaLoader {
    fragments: Vec<SchemaFragment>,
}

impl CompoundSchemaLoader {
    pub fn load(mut self, loader: impl SchemaLoader) -> Self {
        self.fragments.extend(loader.load());
        self
    }
}

impl TryInto<Schema> for CompoundSchemaLoader {
    type Error = SchemaError;

    fn try_into(self) -> Result<Schema, Self::Error> {
        let schema = Schema::from_schema_fragments(self.fragments)?;
        Ok(schema)
    }
}
