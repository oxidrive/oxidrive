use async_trait::async_trait;
use oxidrive_domain::make_error_wrapper;

use super::{PersonalAccessToken, Token};

pub use pg::*;
pub use sqlite::*;

mod pg;
mod sqlite;

make_error_wrapper!(ByTokenError);
make_error_wrapper!(SaveError);

#[mockall::automock]
#[async_trait]
pub trait PersonalAccessTokenStore: Send + Sync + 'static {
    async fn by_token(&self, token: Token) -> Result<Option<PersonalAccessToken>, ByTokenError>;
    async fn save(&self, token: PersonalAccessToken) -> Result<PersonalAccessToken, SaveError>;
}

#[cfg(test)]
mod tests;
