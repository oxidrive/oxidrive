use cedar_policy::{
    entities_errors::EntitiesError, Context, Entities, Entity, EntityId, EntityUid, PolicySet,
    Request, Schema,
};
use serde_json::json;

use crate::{Authorized, Ref};

pub mod policies;
pub mod schema;

#[derive(Clone)]
pub struct CedarAuthorizer {
    policies: PolicySet,
    schema: Schema,
    inner: cedar_policy::Authorizer,
}

impl CedarAuthorizer {
    pub fn new<P, S>(policies: P, schema: S) -> Self
    where
        P: TryInto<PolicySet>,
        P::Error: std::error::Error,

        S: TryInto<Schema>,
        S::Error: std::error::Error,
    {
        Self {
            policies: policies.try_into().unwrap(),
            schema: schema.try_into().unwrap(),
            inner: cedar_policy::Authorizer::new(),
        }
    }

    pub fn authorize<Principal, Action, Resource>(
        &self,
        principal: &Principal,
        action: Action,
        resource: &Resource,
    ) -> Authorized
    where
        Principal: crate::Entity,
        Action: AsRef<str> + Send,
        Resource: crate::Entity,
    {
        let context = Context::empty();

        let principal = try_entity(principal, &self.schema).unwrap_or_else(report_error);
        let action = EntityUid::from_type_name_and_id(
            format!("{}::Action", Resource::TYPE)
                .parse()
                .unwrap_or_else(report_error),
            EntityId::new(action),
        );
        let resource = try_entity(resource, &self.schema).unwrap_or_else(report_error);

        let request = Request::new(
            principal.uid(),
            action.clone(),
            resource.uid(),
            context,
            Some(&self.schema),
        )
        .unwrap();

        let p = principal.uid();
        let r = resource.uid();

        let entities = Entities::from_entities([principal, resource], Some(&self.schema))
            .unwrap_or_else(report_error);

        let response = self
            .inner
            .is_authorized(&request, &self.policies, &entities);

        tracing::trace!(target: "oxidrive::authorizer",
            principal = %p, %action, resource = %r,
            decision = ?response.decision(),
            diagnostics = ?response.diagnostics(),
            "authorization request evaluated",
        );

        response.into()
    }
}

impl From<cedar_policy::Response> for Authorized {
    fn from(response: cedar_policy::Response) -> Self {
        match response.decision() {
            cedar_policy::Decision::Allow => Self::Allow,
            cedar_policy::Decision::Deny => Self::Deny,
        }
    }
}

fn try_entity<E: crate::Entity>(entity: &E, schema: &Schema) -> Result<Entity, EntitiesError> {
    let json = json!({
        "uid": Ref::from(entity),
        "attrs": entity.attrs(),
        "parents": entity.parents(),
    });

    Entity::from_json_value(json, Some(schema))
}

fn report_error<R>(err: impl miette::Diagnostic) -> R {
    panic!("{err} ({err:?})")
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use assert2::let_assert;
    use serde::Serialize;

    use crate::{Entity, Ref};

    use super::*;

    #[derive(Debug, Serialize)]
    struct TestAccount {
        id: &'static str,
    }

    impl Entity for TestAccount {
        const TYPE: &'static str = "Account";

        fn id(&self) -> String {
            self.id.into()
        }

        fn attrs(&self) -> impl Serialize {
            self
        }
    }

    #[derive(Debug, Serialize)]
    struct TestResource {
        id: &'static str,
        owner: Ref<TestAccount>,
    }

    impl Entity for TestResource {
        const TYPE: &'static str = "Resource";

        fn id(&self) -> String {
            self.id.into()
        }

        fn attrs(&self) -> impl Serialize {
            self
        }
    }

    const POLICY: &str = r#"
permit(
    principal is Account,
    action == Resource::Action::"get",
    resource is Resource
) when {
  resource.owner == principal
};
"#;

    const SCHEMA: &str = r#"
entity Account {
  id: String,
};

entity Resource {
  id: String,
  owner: Account,
};

namespace Resource {
  action get appliesTo {
    principal: Account,
    resource: Resource,
  };

  action delete appliesTo {
    principal: Account,
    resource: Resource,
  };
}
"#;

    #[test]
    fn it_authorizes_a_request() {
        let account = TestAccount { id: "test-account" };
        let resource = TestResource {
            id: "test-resource",
            owner: (&account).into(),
        };

        let policies = PolicySet::from_str(POLICY).unwrap();

        let (schema, warnings) = Schema::from_cedarschema_str(SCHEMA).unwrap();
        for warning in warnings {
            eprintln!("WARNING: {warning}");
        }

        let authorizer = CedarAuthorizer::new(policies, schema);

        let response = authorizer.authorize(&account, "get", &resource);
        let_assert!(Authorized::Allow = response);

        let response = authorizer.authorize(&account, "delete", &resource);
        let_assert!(Authorized::Deny = response);
    }
}
