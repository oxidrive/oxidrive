use std::sync::Arc;

use time::OffsetDateTime;

use crate::account::AccountId;

use super::{
    store::{ByTokenError, PersonalAccessTokenStore, SaveError},
    InvalidExpirationDate, PersonalAccessToken, Token,
};

#[derive(Clone)]
pub struct PersonalAccessTokens {
    tokens: Arc<dyn PersonalAccessTokenStore>,
}

impl PersonalAccessTokens {
    pub fn new(tokens: Arc<dyn PersonalAccessTokenStore>) -> Self {
        Self { tokens }
    }

    pub async fn by_token(
        &self,
        token: Token,
    ) -> Result<Option<PersonalAccessToken>, ByTokenError> {
        self.tokens.by_token(token).await
    }

    pub async fn create(
        &self,
        account_id: AccountId,
        expires_at: Option<OffsetDateTime>,
    ) -> Result<(Token, PersonalAccessToken), CreatePatError> {
        let (token, pat) = PersonalAccessToken::new(account_id);
        let pat = pat.expiring(expires_at)?;
        let pat = self.tokens.save(pat).await?;
        Ok((token, pat))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreatePatError {
    #[error(transparent)]
    InvalidExpirationDate(#[from] InvalidExpirationDate),
    #[error(transparent)]
    SaveFailed(#[from] SaveError),
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};
    use time::Duration;

    use crate::pat::store::MockPersonalAccessTokenStore;

    use super::*;

    #[tokio::test]
    async fn it_creates_a_token() {
        let account_id = AccountId::new();
        let expires_at = Some(OffsetDateTime::now_utc() + Duration::days(1));

        let mut store = MockPersonalAccessTokenStore::new();
        store
            .expect_save()
            .withf(move |pat| pat.account_id == account_id && pat.expires_at == expires_at)
            .returning(Ok);

        let pats = PersonalAccessTokens::new(Arc::new(store));

        let (token, created) = pats.create(account_id, expires_at).await.unwrap();
        check!(created.account_id == account_id);
        check!(created.expires_at == expires_at);
        check!(created.verify(token));
    }

    #[tokio::test]
    async fn it_does_not_create_a_token_with_an_invalid_expiration_date() {
        let account_id = AccountId::new();
        let expires_at = Some(OffsetDateTime::now_utc() - Duration::days(1));

        let mut store = MockPersonalAccessTokenStore::new();
        store
            .expect_save()
            .withf(move |pat| pat.account_id == account_id && pat.expires_at == expires_at)
            .returning(Ok);

        let pats = PersonalAccessTokens::new(Arc::new(store));

        let_assert!(Err(err) = pats.create(account_id, expires_at).await);
        let_assert!(CreatePatError::InvalidExpirationDate(_) = err);
    }
}
