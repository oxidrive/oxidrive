use std::str::FromStr;

use assert2::{check, let_assert};

use crate::{
    account::{AccountId, macros::account_id},
    pat::{PersonalAccessToken, Token, TokenId, macros::token_id},
};

use super::PersonalAccessTokenStore;

macro_rules! check_pat {
    ($expected:expr, $actual:expr) => {
        check!($expected.id == $actual.id);
        check!($expected.token_hash == $actual.token_hash);
        check!($expected.account_id == $actual.account_id);
        check!($expected.expires_at == $actual.expires_at);
    };
}

const ACCOUNT_ID: AccountId = account_id!("0194327d-becc-7ef3-809c-35dd09f62f45");

const PAT: &str = "oxipat.lrmAlbDsZ5NZB53gVva-WQ";
const PAT_ID: TokenId = token_id!("0194fffe-ba96-7b43-82fc-ef0e3168d316");

const EXPIRED_PAT: &str = "oxipat.AG8p4eATLlKj2dJwJzcqoQ";
const _EXPIRED_PAT_ID: TokenId = token_id!("0194fffb-b8a2-7451-8035-2a961413f57c");

async fn store_token<S: PersonalAccessTokenStore>(store: S) {
    let (token, pat) = PersonalAccessToken::new(ACCOUNT_ID);

    let saved = store.save(pat.clone()).await.unwrap();
    check_pat!(pat, saved);

    let found = store.by_token(token).await.unwrap();
    let_assert!(Some(found) = found);
    check_pat!(pat, found);
    check!(found.verify(token));
}

async fn exclude_expired_tokens<S: PersonalAccessTokenStore>(store: S) {
    let token = Token::from_str(PAT).unwrap();
    let_assert!(Some(found) = store.by_token(token).await.unwrap());
    check!(found.id == PAT_ID);
    check!(found.account_id == ACCOUNT_ID);

    let token = Token::from_str(EXPIRED_PAT).unwrap();
    let_assert!(None = store.by_token(token).await.unwrap());
}

mod pg {
    use oxidrive_database::migrate::PG_MIGRATOR;

    use crate::pat::store::pg::PgPersonalAccessTokens;

    use super::*;

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures("../../fixtures/postgres/accounts.sql",)
    )]
    async fn it_lists_all_files(pool: sqlx::PgPool) {
        let store = PgPersonalAccessTokens::new(pool);
        store_token(store).await;
    }

    #[sqlx::test(
        migrator = "PG_MIGRATOR",
        fixtures(
            "../../fixtures/postgres/accounts.sql",
            "../../fixtures/postgres/pats.sql"
        )
    )]
    async fn it_excludes_expired_tokens(pool: sqlx::PgPool) {
        let store = PgPersonalAccessTokens::new(pool);
        exclude_expired_tokens(store).await;
    }
}

mod sqlite {
    use oxidrive_database::migrate::SQLITE_MIGRATOR;

    use crate::pat::store::SqlitePersonalAccessTokens;

    use super::*;

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures("../../fixtures/sqlite/accounts.sql",)
    )]
    async fn it_lists_all_files(pool: sqlx::SqlitePool) {
        let store = SqlitePersonalAccessTokens::new(pool);
        store_token(store).await;
    }

    #[sqlx::test(
        migrator = "SQLITE_MIGRATOR",
        fixtures("../../fixtures/sqlite/accounts.sql", "../../fixtures/sqlite/pats.sql")
    )]
    async fn it_excludes_expired_tokens(pool: sqlx::SqlitePool) {
        let store = SqlitePersonalAccessTokens::new(pool);
        exclude_expired_tokens(store).await;
    }
}
