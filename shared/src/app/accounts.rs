use super::subscriptions::Subscriptions;
use opml::OPML;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Accounts {
    pub accts: Vec<Account>,
}

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

#[derive(Debug, Error)]
pub enum Error {
    #[error("{action} \"{item}\". {reason}")]
    AlreadyExists {
        action: String,
        item: String,
        reason: String,
    },
}

// FIXME make this do proper stuff such as platform checks and authehtication
impl Account {
    fn set_account_name(account_type: &AccountType) -> String {
        match account_type {
            AccountType::Local => "On Device".to_string(),
            AccountType::Apple => "iCloud".to_string(),
            AccountType::Google => "Google Sync".to_string(),
            AccountType::Microsoft => "Live 365".to_string(),
            AccountType::Canonical => "Ubuntu One".to_string(),
        }
    }

    fn new(account_type: &AccountType) -> Self {
        Account {
            name: Self::set_account_name(&account_type),
            subs: Subscriptions {
                opml: OPML::default(),
            },
        }
    }
}

impl Accounts {
    // ANCHOR: helper functions
    fn set_duplicate_err(action: &str, item: &str, reason: &str) -> self::Error {
        self::Error::AlreadyExists {
            action: action.to_string(),
            item: item.to_string(),
            reason: reason.to_string(),
        }
    }
    // ANCHOR_END: helper functions

    pub fn add_account(&self, account_type: &AccountType) -> Result<Self, self::Error> {
        let mut accounts = self.clone();
        let account_to_add = Account::new(&account_type);
        let duplicate_err = Self::set_duplicate_err(
            "Cannot add account",
            account_to_add.name.as_str(),
            "It already exists.",
        );

        if accounts.accts.contains(&account_to_add) {
            Err(duplicate_err)
        } else {
            accounts.accts.push(account_to_add);
            Ok(accounts)
        }
    }

    pub fn delete_account(&self, account_type: &AccountType) -> Self {
        let mut accounts = self.clone();
        let delete_account_name = Account::set_account_name(&account_type);

        accounts
            .accts
            .retain(|account| account.name != delete_account_name);
        accounts
    }
}

#[cfg(test)]
mod accounts {
    use super::*;
    use crate::{CrabNews, Event, Model};
    use crux_core::testing::AppTester;

    #[test]
    fn add_new_local_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let does_contain_account = model.accounts.accts.contains(&account_to_add);

        assert_eq!(does_contain_account, true);
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
        let _ = app.update(Event::DeleteAccount(AccountType::Local), &mut model);

        let does_contain_account = model.accounts.accts.contains(&account_to_delete);

        assert_eq!(does_contain_account, false);
    }

    #[test]
    fn add_new_platform_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Apple);

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model);
        let does_contain_account = model.accounts.accts.contains(&account_to_add);

        assert_eq!(does_contain_account, true);
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
        let _ = app.update(Event::DeleteAccount(AccountType::Apple), &mut model);

        let does_contain_account = model.accounts.accts.contains(&account_to_delete);

        assert_eq!(does_contain_account, false);
    }
}
