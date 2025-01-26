use std::{collections::HashMap, marker::PhantomData};

use cedar::{policies::CompoundPolicyLoader, schema::CompoundSchemaLoader};
use serde::{ser::SerializeMap, Serialize};

pub mod cedar;

pub use cedar::CedarAuthorizer as Authorizer;

pub enum Authorized {
    Allow,
    Deny,
}

impl Authorized {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allow)
    }

    pub fn into_err<E: From<Self>>(self) -> Result<(), E> {
        self.into()
    }
}

impl<E: From<Authorized>> From<Authorized> for Result<(), E> {
    fn from(value: Authorized) -> Self {
        match value {
            Authorized::Allow => Ok(()),
            Authorized::Deny => Err(E::from(value)),
        }
    }
}

pub trait Entity: Send + Sync {
    const TYPE: &'static str;

    fn id(&self) -> String;

    fn attrs(&self) -> impl Serialize;

    fn parents(&self) -> Vec<AnyRef> {
        vec![]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ref<E: Entity> {
    inner: AnyRef,
    _entity: PhantomData<E>,
}

impl<E: Entity> Ref<E> {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            inner: AnyRef::new(E::TYPE, id),
            _entity: PhantomData,
        }
    }

    pub fn type_(&self) -> &str {
        self.inner.type_()
    }

    pub fn id(&self) -> &str {
        self.inner.id()
    }
}

impl<E: Entity> From<&E> for Ref<E> {
    fn from(entity: &E) -> Self {
        Self::new(entity.id())
    }
}

impl<E: Entity> Serialize for Ref<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<E: Entity> Clone for Ref<E> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _entity: self._entity,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnyRef {
    r#type: &'static str,
    id: String,
}

impl AnyRef {
    pub fn new(r#type: &'static str, id: impl Into<String>) -> Self {
        Self {
            r#type,
            id: id.into(),
        }
    }

    pub fn type_(&self) -> &str {
        self.r#type
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<E: Entity> From<&E> for AnyRef {
    fn from(entity: &E) -> Self {
        Self::new(E::TYPE, entity.id())
    }
}

impl Serialize for AnyRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let entity = HashMap::from([("type", self.r#type.to_string()), ("id", self.id.clone())]);

        let mut container = serializer.serialize_map(Some(1))?;
        container.serialize_entry("__entity", &entity)?;
        container.end()
    }
}

impl<E: Entity> From<Ref<E>> for AnyRef {
    fn from(value: Ref<E>) -> Self {
        value.inner
    }
}

pub struct AuthorizationModule;

impl app::Module for AuthorizationModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(
            |policies: CompoundPolicyLoader, schema: CompoundSchemaLoader| {
                Authorizer::new(policies, schema)
            },
        );
    }
}
