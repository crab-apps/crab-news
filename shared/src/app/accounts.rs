use super::subscriptions::Subscriptions;
use super::Error;
use crate::define_newtype;

use serde::{Deserialize, Serialize};

define_newtype!(OldAccountName);
define_newtype!(NewAccountName);

// TODO add more fields?
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct Account {
    pub name: String,
    pub subs: Subscriptions,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum AccountType {
    Local,
    Apple,
    Google,
    Microsoft,
    Canonical,
    // TODO add cloud accounts
}

// FIXME make this do proper stuff such as platform checks and authentication
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

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Accounts {
    pub acct: Vec<Account>,
}

impl Accounts {
    // ANCHOR: helper functions
    fn set_error(action: &str, item: &str, reason: &str) -> self::Error {
        self::Error::CrabError {
            action: action.to_string(),
            item: item.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn find_account_index(&self, account: &Account) -> usize {
        self.acct
            .iter()
            .position(|a| a.name == account.name)
            .unwrap()
    }

    pub fn add_account(&self, account_type: &AccountType) -> Result<Self, self::Error> {
        let mut accounts = self.clone();
        let account_to_add = Account::new(account_type);
        let duplicate_err = Self::set_error(
            "Cannot add account",
            account_to_add.name.as_str(),
            "It already exists.",
        );

        if accounts.acct.contains(&account_to_add) {
            Err(duplicate_err)
        } else {
            accounts.acct.push(account_to_add);
            Ok(accounts)
        }
    }

    pub fn delete_account(&self, account: &Account) -> Self {
        let mut accounts = self.clone();
        accounts.acct.retain(|a| a.name != account.name);
        accounts
    }

    pub fn rename_account(
        &self,
        old_account_name: &OldAccountName,
        new_account_name: &NewAccountName,
    ) -> Result<Self, self::Error> {
        let mut accounts = self.clone();
        let doesnt_exist_err = Self::set_error(
            "Cannot rename account",
            old_account_name.0.as_str(),
            "It doesn't exists.",
        );

        for a in accounts.acct.iter_mut() {
            if a.name == old_account_name.to_string() {
                a.name = new_account_name.to_string();
            } else {
                return Err(doesnt_exist_err);
            }
        }

        Ok(accounts)
    }
}

#[cfg(test)]
mod accts {
    use super::*;
    use crate::{CrabNews, Event, Model};
    use crux_core::App;

    #[test]
    fn add_new_local_account() {
        let app = CrabNews;
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let does_contain_account = model.accounts.acct.contains(&account_to_add);

        assert!(does_contain_account);
    }

    #[test]
    fn fail_new_local_account() {
        let app = CrabNews;
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot add account \"{}\". It already exists.",
            account_to_add.name
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn delete_local_account() {
        let app = CrabNews;
        let mut model = Model::default();
        let account_to_delete = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let _ = app.update(
            Event::DeleteAccount(account_to_delete.clone()),
            &mut model,
            &(),
        );

        let does_contain_account = model.accounts.acct.contains(&account_to_delete);

        assert!(!does_contain_account);
    }

    #[test]
    fn add_new_platform_account() {
        let app = CrabNews;
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Apple);

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model, &());
        let does_contain_account = model.accounts.acct.contains(&account_to_add);

        assert!(does_contain_account);
    }

    #[test]
    fn fail_new_platform_account() {
        let app = CrabNews;
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Apple);

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model, &());
        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model, &());

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot add account \"{}\". It already exists.",
            account_to_add.name
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn delete_platform_account() {
        let app = CrabNews;
        let mut model = Model::default();
        let account_to_delete = Account::new(&AccountType::Apple);

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model, &());
        let _ = app.update(
            Event::DeleteAccount(account_to_delete.clone()),
            &mut model,
            &(),
        );

        let does_contain_account = model.accounts.acct.contains(&account_to_delete);

        assert!(!does_contain_account);
    }

    #[test]
    fn rename_platform_account() {
        let app = CrabNews;
        let mut model = Model::default();
        let old_account_name = OldAccountName("iCloud".to_string());
        let new_account_name = NewAccountName("New Name".to_string());

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model, &());
        let _ = app.update(
            Event::RenameAccount(old_account_name, new_account_name.clone()),
            &mut model,
            &(),
        );

        assert_eq!(
            new_account_name,
            NewAccountName(model.accounts.acct[0].name.clone().to_string())
        );
    }

    #[test]
    fn fail_rename_platform_account_with_empty_name() {
        let app = CrabNews;
        let mut model = Model::default();
        let old_account_name = OldAccountName("".to_string());
        let new_account_name = NewAccountName("New Name".to_string());

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model, &());
        let _ = app.update(
            Event::RenameAccount(old_account_name.clone(), new_account_name),
            &mut model,
            &(),
        );

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot rename account \"{}\". It doesn't exists.",
            old_account_name.0.as_str()
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn fail_rename_platform_account_with_wrong_name() {
        let app = CrabNews;
        let mut model = Model::default();
        let old_account_name = OldAccountName("Dada".to_string());
        let new_account_name = NewAccountName("New Name".to_string());

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model, &());
        let _ = app.update(
            Event::RenameAccount(old_account_name.clone(), new_account_name),
            &mut model,
            &(),
        );

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot rename account \"{}\". It doesn't exists.",
            old_account_name.0.as_str()
        );

        assert_eq!(actual_error, expected_error);
    }
}
