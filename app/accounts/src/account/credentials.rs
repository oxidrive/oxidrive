use std::{collections::HashMap, fmt::Debug};

use argon2::PasswordVerifier;

use super::AccountId;

mod store;

pub use store::*;

#[derive(Clone, Debug)]
pub struct Credentials {
    pub account_id: AccountId,
    pub(crate) creds: HashMap<String, Creds>,
}

impl Credentials {
    pub fn new(account_id: AccountId) -> Self {
        Self {
            account_id,
            creds: Default::default(),
        }
    }

    pub fn add(&mut self, credentials: impl Into<Creds>) -> Result<(), AddError> {
        let creds = credentials.into();
        let id = creds.id();

        if self.creds.contains_key(id) {
            return Err(AddError::AlreadyPresent);
        }

        self.creds.insert(id.into(), creds);
        Ok(())
    }

    pub fn replace(&mut self, credentials: impl Into<Creds>) {
        let creds = credentials.into();
        let id = creds.id();
        self.creds.insert(id.into(), creds);
    }

    pub fn verify(&self, credentials: VerifyCreds) -> Result<(), InvalidCredentials> {
        if !self.creds.values().any(|creds| creds.matches(&credentials)) {
            return Err(InvalidCredentials);
        }

        Ok(())
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &Creds> {
        self.creds.values()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AddError {
    #[error("credentials already registered")]
    AlreadyPresent,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid credentials")]
pub struct InvalidCredentials;

#[derive(Clone, Debug, PartialEq)]
pub enum Creds {
    Password(Password),
}

impl Creds {
    pub(crate) fn id(&self) -> &str {
        match self {
            Creds::Password(_) => "password",
        }
    }

    fn matches(&self, other: &VerifyCreds) -> bool {
        match (self, other) {
            (Creds::Password(hash), VerifyCreds::Password(pwd)) => hash.verify(pwd),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Password {
    pub(crate) password_hash: String,
}

impl Password {
    pub fn from_hash(password_hash: String) -> Self {
        Self { password_hash }
    }

    pub fn hash(password: impl AsRef<[u8]>) -> Result<Self, HashError> {
        use argon2::{
            Argon2,
            password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
        };

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_ref(), &salt)?
            .to_string();
        Ok(Self { password_hash })
    }

    pub fn verify(&self, password: impl AsRef<[u8]> + Debug) -> bool {
        use argon2::{Argon2, PasswordHash};

        let hash = PasswordHash::new(&self.password_hash).unwrap();

        Argon2::default()
            .verify_password(password.as_ref(), &hash)
            .is_ok()
    }
}

impl Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Password").finish()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("password hashing failed")]
pub struct HashError(#[from] argon2::password_hash::Error);

impl From<Password> for Creds {
    fn from(value: Password) -> Self {
        Self::Password(value)
    }
}

pub enum VerifyCreds {
    Password(String),
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use fake::Fake;
    use rstest::fixture;

    use crate::account::{Account, fixtures::account};

    use super::*;

    #[fixture]
    pub fn password() -> String {
        fake::faker::internet::en::Password(8..12).fake()
    }

    #[fixture]
    pub fn credentials(account: Account) -> Credentials {
        Credentials::new(account.id)
    }

    #[fixture]
    pub fn with_password(account: Account, password: String) -> Credentials {
        let mut c = Credentials::new(account.id);
        c.add(Password::hash(password).unwrap()).unwrap();
        c
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::fixtures::*;
    use super::*;

    #[rstest]
    fn it_adds_a_new_credential(mut credentials: Credentials, password: String) {
        credentials
            .add(Password::hash(password.clone()).unwrap())
            .unwrap();

        credentials.verify(VerifyCreds::Password(password)).unwrap();

        credentials
            .verify(VerifyCreds::Password("wrong password".into()))
            .unwrap_err();
    }

    #[rstest]
    fn it_replaces_a_credential(mut credentials: Credentials, password: String) {
        credentials
            .add(Password::hash(password.clone()).unwrap())
            .unwrap();

        credentials
            .verify(VerifyCreds::Password(password.clone()))
            .unwrap();

        credentials.replace(Password::hash("changed").unwrap());

        credentials
            .verify(VerifyCreds::Password("changed".into()))
            .unwrap();

        credentials
            .verify(VerifyCreds::Password(password))
            .unwrap_err();
    }
}
