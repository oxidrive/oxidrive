use std::{fmt::Display, str::FromStr, sync::Arc};

use base64::prelude::*;
use oxidrive_database::Database;
use oxidrive_domain::make_uuid_type;
pub use service::*;
use store::{PersonalAccessTokenStore, PgPersonalAccessTokens, SqlitePersonalAccessTokens};
use time::OffsetDateTime;

use crate::account::AccountId;

mod service;
mod store;

pub static PREFIX: &str = "oxipat.";

make_uuid_type!(TokenId, token_id);

#[derive(Clone, Debug)]
pub struct PersonalAccessToken {
    pub id: TokenId,
    token_hash: blake3::Hash,
    pub account_id: AccountId,
    pub expires_at: Option<OffsetDateTime>,
}

impl PersonalAccessToken {
    pub fn new(account_id: AccountId) -> (Token, Self) {
        let token = Token::new();

        (
            token,
            Self {
                id: TokenId::new(),
                token_hash: token.hashed(),
                account_id,
                expires_at: None,
            },
        )
    }

    pub fn expiring(
        mut self,
        expires_at: Option<OffsetDateTime>,
    ) -> Result<Self, InvalidExpirationDate> {
        if let Some(expires_at) = expires_at {
            if expires_at < OffsetDateTime::now_utc() {
                return Err(InvalidExpirationDate);
            }
        }

        self.expires_at = expires_at;
        Ok(self)
    }

    pub fn verify(&self, token: Token) -> bool {
        let hash = token.hashed();
        self.token_hash == hash
    }
}

#[derive(Debug, thiserror::Error)]
#[error("token expiration date must be in the future")]
pub struct InvalidExpirationDate;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token(i128);

impl Token {
    fn new() -> Self {
        use rand::prelude::*;

        Self(rand::thread_rng().gen())
    }

    fn hashed(&self) -> blake3::Hash {
        blake3::hash(&self.0.to_le_bytes())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut encoded = [0; 22];
        BASE64_URL_SAFE_NO_PAD
            .encode_slice(self.0.to_le_bytes(), &mut encoded)
            .expect("encoded token must be exactly 22 bytes");
        let encoded = std::str::from_utf8(&encoded).expect("encoded token must be valid UTF-8");

        write!(f, "{PREFIX}{encoded}")
    }
}

impl FromStr for Token {
    type Err = TokenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix(PREFIX) else {
            return Err(TokenParseError::NotAToken);
        };

        let mut decoded = [0; 16];
        let bytes_decoded = BASE64_URL_SAFE_NO_PAD.decode_slice(s.as_bytes(), &mut decoded)?;
        if bytes_decoded != 16 {
            let err = base64::DecodeError::InvalidLength(bytes_decoded);
            return Err(base64::DecodeSliceError::DecodeError(err).into());
        }

        Ok(Self(i128::from_le_bytes(decoded)))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TokenParseError {
    #[error("not a personal access token")]
    NotAToken,
    #[error(transparent)]
    DecodeError(#[from] base64::DecodeSliceError),
}

pub struct PersonalAccessTokensModule;

impl app::Module for PersonalAccessTokensModule {
    fn mount(self: Box<Self>, c: &mut app::di::Context) {
        c.bind(store);
        c.bind(PersonalAccessTokens::new);
    }
}

fn store(database: Database) -> Arc<dyn PersonalAccessTokenStore> {
    match database {
        Database::Sqlite(pool) => Arc::new(SqlitePersonalAccessTokens::new(pool)),
        Database::Pg(pool) => Arc::new(PgPersonalAccessTokens::new(pool)),
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;

    use super::*;

    #[test]
    fn it_generates_a_prefixed_secure_string() {
        let (token, pat) = PersonalAccessToken::new(AccountId::new());
        let token_s = token.to_string();
        check!(token_s.to_string().starts_with(PREFIX));

        let parsed = token_s.parse::<Token>().unwrap();
        check!(parsed == token);

        check!(pat.verify(token));
    }
}
