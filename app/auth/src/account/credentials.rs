use std::collections::HashMap;

use argon2::PasswordVerifier;

use super::AccountId;

mod store;

pub use store::*;

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

pub enum Creds {
    Password(Password),
}

impl Creds {
    pub(crate) fn id(&self) -> &str {
        match self {
            Creds::Password(_) => "password",
        }
    }

    pub(crate) fn kind(&self) -> &str {
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
            password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
            Argon2,
        };

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_ref(), &salt)?
            .to_string();
        Ok(Self { password_hash })
    }

    pub fn verify(&self, password: impl AsRef<[u8]>) -> bool {
        use argon2::{Argon2, PasswordHash};

        let hash = PasswordHash::new(&self.password_hash).unwrap();

        Argon2::default()
            .verify_password(password.as_ref(), &hash)
            .is_ok()
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

    use crate::account::{fixtures::account, Account};

    use super::*;

    #[fixture]
    pub fn password() -> String {
        fake::faker::internet::en::Password(8..12).fake()
    }

    #[fixture]
    pub fn credentials(account: Account) -> Credentials {
        Credentials::new(account.id)
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
}
