use super::subscriptions::Subscriptions;
use super::Error;
use opml::OPML;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Accounts {
    pub accts: Vec<Account>,
}

// TODO add more fields?
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub subs: Subscriptions,
}

// TODO use crux_platform instead
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountType {
    Local,
    Apple,
    Google,
    Microsoft,
    Canonical,
    // TODO add cloud accounts
}

impl Account {
    // TODO use crux_platform instead
    // FIXME for mow return a string BUT make this do proper stuff such as
    // platform checks
    // auth token
    // more
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
            id: Uuid::new_v4(),
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
        let account_name = Account::set_account_name(&account_type);
        let duplicate_err = Self::set_duplicate_err(
            "Cannot add account",
            account_name.as_str(),
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
        let account_to_delete = Account::set_account_name(&account_type);

        accounts
            .accts
            .retain(|account| account.name != account_to_delete);
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
        let added_account = Account {
            id: Uuid::new_v4(),
            name: "On Device".to_string(),
            subs: Subscriptions {
                opml: OPML::default(),
            },
        };

        let _ = app.update(Event::AddAccount(AccountType::Local), &mut model);
        let does_contain_account = model.accounts.accts.contains(&added_account);

        assert_eq!(does_contain_account, true);
    }

    #[test]
    fn fail_new_local_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_name = "On Device".to_string();

        let _ = app.update(Event::AddAccount(AccountType::Local), &mut model);
        let _ = app.update(Event::AddAccount(AccountType::Local), &mut model);

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot add account \"{}\". It already exists.",
            account_name
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn delete_local_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let deleted_account = Account {
            id: Uuid::new_v4(),
            name: "On Device".to_string(),
            subs: Subscriptions {
                opml: OPML::default(),
            },
        };

        let _ = app.update(Event::AddAccount(AccountType::Local), &mut model);
        let _ = app.update(Event::DeleteAccount(AccountType::Local), &mut model);

        let does_contain_account = model.accounts.accts.contains(&deleted_account);

        assert_eq!(does_contain_account, false);
    }

    //     #[test]
    //     fn add_new_native_account() {
    //         let app = AppTester::<CrabNews, _>::default();
    //         let mut model = Model::default();
    //         let added_account = Account {
    //             id: Uuid::new_v4(),
    //             name: "iCloud".to_string(),
    //             subs: Subscriptions {
    //                 opml: OPML::default(),
    //             },
    //         };

    //         let _ = app.update(Event::AddAccount(AccountType::Apple), &mut model);
    //         let does_contain_account = model.accounts.contains(&added_account);

    //         assert_eq!(does_contain_account, true);
    //     }

    //     #[test]
    //     fn fail_new_native_account() {
    //         let app = AppTester::<CrabNews, _>::default();
    //         let mut model = Model::default();
    //         let account_name = "iCloud".to_string();

    //         let _ = app.update(Event::AddAccount(AccountType::Apple), &mut model);
    //         let _ = app.update(Event::AddAccount(AccountType::Apple), &mut model);

    //         let actual_error = model.notification.message;
    //         let expected_error = format!(
    //             "Cannot add account \"{}\". It already exists.",
    //             account_name
    //         );

    //         assert_eq!(actual_error, expected_error);
    //     }

    //     #[test]
    //     fn delete_native_account() {
    //         let app = AppTester::<CrabNews, _>::default();
    //         let mut model = Model::default();
    //         let deleted_account = Account {
    //             id: Uuid::new_v4(),
    //             name: "iCloud".to_string(),
    //             subs: Subscriptions {
    //                 opml: OPML::default(),
    //             },
    //         };

    //         let _ = app.update(Event::AddAccount(AccountType::Apple), &mut model);
    //         let _ = app.update(Event::DeleteAccount(AccountType::Apple), &mut model);

    //         let does_contain_account = model.accounts.contains(&deleted_account);

    //         assert_eq!(does_contain_account, false);
    //     }
}
