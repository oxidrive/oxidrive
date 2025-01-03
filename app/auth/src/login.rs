use std::sync::Arc;

use crate::account::{Account, AccountCredentials, Accounts, VerifyCreds};

#[derive(Clone)]
pub struct Login {
    pub(crate) accounts: Arc<dyn Accounts>,
    pub(crate) credentials: Arc<dyn AccountCredentials>,
}

impl Login {
    pub async fn password(
        &self,
        username: &str,
        password: impl Into<String>,
    ) -> Result<Account, AuthenticationFailed> {
        let Some(account) = self
            .accounts
            .by_username(username)
            .await
            .map_err(|_| AuthenticationFailed)?
        else {
            return Err(AuthenticationFailed);
        };

        let credentials = self
            .credentials
            .for_account(account.id)
            .await
            .map_err(|_| AuthenticationFailed)?;

        credentials
            .verify(VerifyCreds::Password(password.into()))
            .map_err(|_| AuthenticationFailed)?;

        Ok(account)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("authentication failed")]
pub struct AuthenticationFailed;
