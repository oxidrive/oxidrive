use oxidrive_domain::make_uuid_type;

mod credentials;
mod store;

pub use credentials::*;
pub use store::*;

make_uuid_type!(AccountId, account_id);

#[derive(Clone, Debug)]
pub struct Account {
    pub id: AccountId,
    pub username: String,
}

impl Account {
    pub fn create(username: impl Into<String>) -> Self {
        Self {
            id: AccountId::new(),
            username: username.into(),
        }
    }
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use fake::Fake;
    use rstest::fixture;

    use super::*;

    #[fixture]
    pub fn account() -> Account {
        Account::create(fake::faker::internet::en::Username().fake::<String>())
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use rstest::rstest;

    use super::fixtures::account;
    use super::*;

    #[rstest]
    fn it_creates_a_new_account(account: Account) {
        let created = Account::create(account.username.clone());

        check!(created.username == account.username);
    }
}
