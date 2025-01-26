use std::sync::Arc;

use crate::account::{
    Account, AccountCredentials, Accounts, ByUsernameError, ForAccountError, VerifyCreds,
};

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
        let Some(account) = self.accounts.by_username(username).await? else {
            return Err(AuthenticationFailed::Unauthorized);
        };

        let credentials = self.credentials.for_account(account.id).await?;

        credentials
            .verify(VerifyCreds::Password(password.into()))
            .map_err(|_| AuthenticationFailed::Unauthorized)?;

        Ok(account)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationFailed {
    #[error(transparent)]
    ByUsernameError(#[from] ByUsernameError),
    #[error(transparent)]
    CredentialsError(#[from] ForAccountError),
    #[error("authentication failed")]
    Unauthorized,
}
