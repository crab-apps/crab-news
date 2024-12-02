use super::subscriptions::Subscriptions;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Accounts = Vec<Account>;

// TODO add more fields?
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Account {
    pub name: String,
    pub subs: Subscriptions,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountType {
    Local,
    Apple,
    Google,
    Microsoft,
    Canonical,
    // TODO add cloud accounts
}

// FIXME make this do proper stuff such as platform checks and authehtication
impl Account {
    fn set_account_name(account_type: &AccountType) -> &str {
        match account_type {
            AccountType::Local => "On Device",
            AccountType::Apple => "iCloud",
            AccountType::Google => "Google Sync",
            AccountType::Microsoft => "Live 365",
            AccountType::Canonical => "Ubuntu One",
        }
    }

    pub fn new(account_type: &AccountType) -> Self {
        Account {
            name: Self::set_account_name(account_type).to_string(),
            subs: Subscriptions::default(),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{action} \"{item}\". {reason}")]
    AlreadyExists {
        action: String,
        item: String,
        reason: String,
    },
}

trait AccountsHelpers {
    fn set_duplicate_err(action: &str, item: &str, reason: &str) -> self::Error;
}

impl AccountsHelpers for Accounts {
    // ANCHOR: helper functions
    fn set_duplicate_err(action: &str, item: &str, reason: &str) -> self::Error {
        self::Error::AlreadyExists {
            action: action.to_string(),
            item: item.to_string(),
            reason: reason.to_string(),
        }
    }
}

pub trait AccountsExt {
    fn find_account_index(&self, account: &Account) -> usize;
    fn add_account(&self, account_type: &AccountType) -> Result<Self, self::Error>
    where
        Self: Sized;
    fn delete_account(&self, account: &Account) -> Self
    where
        Self: Sized;
}

impl AccountsExt for Accounts {
    fn find_account_index(&self, account: &Account) -> usize {
        self.iter().position(|a| a.name == account.name).unwrap()
    }

    fn add_account(&self, account_type: &AccountType) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let account_to_add = Account::new(account_type);
        let duplicate_err = Self::set_duplicate_err(
            "Cannot add account",
            account_to_add.name.as_str(),
            "It already exists.",
        );

        if subs.contains(&account_to_add) {
            Err(duplicate_err)
        } else {
            subs.push(account_to_add);
            Ok(subs)
        }
    }

    fn delete_account(&self, account: &Account) -> Self {
        let mut subs = self.clone();
        subs.retain(|a| a.name != account.name);
        subs
    }
}

#[cfg(test)]
mod accts {
    use super::*;
    use crate::{CrabNews, Event, Model};
    use crux_core::testing::AppTester;

    #[test]
    fn add_new_local_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let does_contain_account = model.accounts.contains(&account_to_add);

        assert!(does_contain_account);
    }

    #[test]
    fn fail_new_local_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot add account \"{}\". It already exists.",
            account_to_add.name
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn delete_local_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_to_delete = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let _ = app.update(Event::DeleteAccount(account_to_delete.clone()), &mut model);

        let does_contain_account = model.accounts.contains(&account_to_delete);

        assert!(!does_contain_account);
    }

    #[test]
    fn add_new_platform_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Apple);

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model);
        let does_contain_account = model.accounts.contains(&account_to_add);

        assert!(does_contain_account);
    }

    #[test]
    fn fail_new_platform_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Apple);

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model);
        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model);

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot add account \"{}\". It already exists.",
            account_to_add.name
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn delete_platform_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_to_delete = Account::new(&AccountType::Apple);

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model);
        let _ = app.update(Event::DeleteAccount(account_to_delete.clone()), &mut model);

        let does_contain_account = model.accounts.contains(&account_to_delete);

        assert!(!does_contain_account);
    }
}
