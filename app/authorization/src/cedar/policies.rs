use cedar_policy::{Policy, PolicyId, PolicySet, PolicySetError};
use rust_embed::Embed;

pub trait Policies: Send + Sync + 'static {
    fn load(&self) -> impl Iterator<Item = Policy>;
}

impl<T> Policies for T
where
    T: Embed + Send + Sync + 'static,
{
    fn load(&self) -> impl Iterator<Item = Policy> {
        Self::iter().map(|path| {
            let file = Self::get(&path).unwrap();
            let src = String::from_utf8_lossy(&file.data);
            let id = PolicyId::new(&path);
            Policy::parse(Some(id), src).unwrap_or_else(|err| {
                panic!("failed to parse Cedar policy {path}: {err} ({err:?})")
            })
        })
    }
}

#[derive(Clone, Default)]
pub struct CompoundPolicyLoader {
    policies: Vec<Policy>,
}

impl CompoundPolicyLoader {
    pub fn load(mut self, loader: impl Policies) -> Self {
        self.policies.extend(loader.load());
        self
    }
}

impl TryInto<PolicySet> for CompoundPolicyLoader {
    type Error = PolicySetError;

    fn try_into(self) -> Result<PolicySet, Self::Error> {
        let policies = PolicySet::from_policies(self.policies)?;
        Ok(policies)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error(transparent)]
    PolicySetError(#[from] PolicySetError),
}
