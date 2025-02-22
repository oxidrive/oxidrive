use std::sync::Arc;

use crate::account::{
    Account, AccountCredentials, ForAccountError, InvalidCredentials, VerifyCreds,
};

use super::{
    ByIdError, DeleteExpiredError, DeleteSessionError, SaveSessionError, Session, SessionId,
    SessionStore,
};

#[derive(Clone)]
pub struct Sessions {
    credentials: Arc<dyn AccountCredentials>,
    sessions: Arc<dyn SessionStore>,
}

impl Sessions {
    pub fn new(credentials: Arc<dyn AccountCredentials>, sessions: Arc<dyn SessionStore>) -> Self {
        Self {
            credentials,
            sessions,
        }
    }

    pub async fn by_id(&self, id: SessionId) -> Result<Option<Session>, ByIdError> {
        self.sessions.by_id(id).await
    }

    pub async fn create(
        &self,
        account: &Account,
        credentials: VerifyCreds,
    ) -> Result<Session, CreateSessionError> {
        let creds = self.credentials.for_account(account.id).await?;
        creds.verify(credentials)?;

        let session = Session::create(account.id);
        let session = self.sessions.save(session).await?;

        Ok(session)
    }

    pub async fn delete(&self, id: SessionId) -> Result<(), DeleteSessionError> {
        self.sessions.delete(id).await
    }

    pub async fn delete_expired(&self) -> Result<Vec<Session>, DeleteExpiredError> {
        self.sessions.delete_expired().await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateSessionError {
    #[error(transparent)]
    LoadCredentialsFailed(#[from] ForAccountError),
    #[error(transparent)]
    InvalidCredentials(#[from] InvalidCredentials),
    #[error(transparent)]
    SaveSessionFailed(#[from] SaveSessionError),
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use rstest::fixture;
    use time::{Duration, OffsetDateTime};

    use crate::account::fixtures::account;

    use super::*;

    #[fixture]
    pub fn session(account: Account) -> Session {
        Session::create(account.id)
    }

    #[fixture]
    pub fn expired(account: Account) -> Session {
        let mut session = Session::create(account.id);
        session.expires_at = OffsetDateTime::now_utc() - Duration::days(1);
        session
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use mockall::predicate::eq;
    use rstest::rstest;

    use crate::{
        account::{
            Account, Credentials, MockAccountCredentials, VerifyCreds,
            credentials::fixtures::{password, with_password},
            fixtures::account,
        },
        session::store::MockSessionStore,
    };

    use super::*;

    #[tokio::test]
    #[rstest]
    async fn it_creates_a_new_session_from_credentials(
        account: Account,
        password: String,
        #[from(with_password)]
        #[with(account.clone(), password.clone())]
        creds: Credentials,
    ) {
        let account_id = account.id;

        let mut credentials = MockAccountCredentials::new();
        credentials
            .expect_for_account()
            .with(eq(account_id))
            .returning(move |_| Ok(creds.clone()));

        let mut sessions = MockSessionStore::new();
        sessions
            .expect_save()
            .withf(move |session| session.account_id == account_id)
            .returning(Ok);

        let sessions = Sessions::new(Arc::new(credentials), Arc::new(sessions));

        let session = sessions
            .create(&account, VerifyCreds::Password(password))
            .await
            .unwrap();

        check!(session.account_id == account.id);
    }
}
