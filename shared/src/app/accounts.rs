use super::subscriptions::Subscriptions;
use super::Error;
use crate::define_newtype;

use serde::{Deserialize, Serialize};

define_newtype!(OldAccountName);
define_newtype!(NewAccountName);

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

// TODO add more fields?
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct Account {
    pub name: String,
    pub subs: Subscriptions,
}

trait AccountHelpers {
    fn set_account_name(account_type: &AccountType) -> String;
}

impl AccountHelpers for Account {
    fn set_account_name(account_type: &AccountType) -> String {
        match account_type {
            AccountType::Local => "On Device",
            AccountType::Apple => "iCloud",
            AccountType::Google => "Google Sync",
            AccountType::Microsoft => "Live 365",
            AccountType::Canonical => "Ubuntu One",
        }
        .to_string()
    }
}

trait NewAccount {
    fn new(account_type: &AccountType) -> Self;
}

// FIXME make this do proper stuff such as platform checks and authentication
impl NewAccount for Account {
    fn new(account_type: &AccountType) -> Self {
        Account {
            name: Self::set_account_name(account_type),
            subs: Subscriptions::default(),
        }
    }
}

impl Account {
    pub fn new(account_type: &AccountType) -> Self {
        <Self as NewAccount>::new(account_type)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Accounts {
    pub acct: Vec<Account>,
}

trait CreateAccount {
    fn create_account(&self, account_type: &AccountType) -> Result<Self, Error>
    where
        Self: Sized;
}

impl CreateAccount for Accounts {
    fn create_account(&self, account_type: &AccountType) -> Result<Self, Error> {
        let mut accounts = self.clone();
        let account_to_add = Account::new(account_type);
        let duplicate_err = Error::set_error(
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
}

trait DeleteAccount {
    fn delete_account(&self, account: &Account) -> Self
    where
        Self: Sized;
}

impl DeleteAccount for Accounts {
    fn delete_account(&self, account: &Account) -> Self {
        let mut accounts = self.clone();
        accounts.acct.retain(|a| a.name != account.name);
        accounts
    }
}

trait RenameAccount {
    fn rename_account(
        &self,
        old_account_name: &OldAccountName,
        new_account_name: &NewAccountName,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

impl RenameAccount for Accounts {
    fn rename_account(
        &self,
        old_account_name: &OldAccountName,
        new_account_name: &NewAccountName,
    ) -> Result<Self, Error> {
        let mut accounts = self.clone();
        let does_not_exist_err = Error::set_error(
            "Cannot rename account",
            old_account_name.0.as_str(),
            "It doesn't exists.",
        );

        for a in accounts.acct.iter_mut() {
            if a.name == old_account_name.to_string() {
                a.name = new_account_name.to_string();
            } else {
                return Err(does_not_exist_err);
            }
        }

        Ok(accounts)
    }
}

trait FindAccount {
    fn find_account_index(&self, account: &Account) -> usize;
}

impl FindAccount for Accounts {
    fn find_account_index(&self, account: &Account) -> usize {
        self.acct
            .iter()
            .position(|a| a.name == account.name)
            .unwrap()
    }
}

impl Accounts {
    pub fn create(&self, account_type: &AccountType) -> Result<Self, Error> {
        <Self as CreateAccount>::create_account(self, account_type)
    }

    pub fn delete(&self, account: &Account) -> Self {
        <Self as DeleteAccount>::delete_account(self, account)
    }

    pub fn rename(
        &self,
        old_account_name: &OldAccountName,
        new_account_name: &NewAccountName,
    ) -> Result<Self, Error> {
        <Self as RenameAccount>::rename_account(self, old_account_name, new_account_name)
    }

    pub fn find_by_index(&self, account: &Account) -> usize {
        <Self as FindAccount>::find_account_index(self, account)
    }
}

#[cfg(test)]
mod accts {
    use super::*;
    use crate::{App, Event, Model};
    use crux_core::App as _;

    #[test]
    fn add_new_local_account() {
        let app = App;
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let does_contain_account = model.accounts.acct.contains(&account_to_add);

        assert!(does_contain_account);
    }

    #[test]
    fn fail_new_local_account() {
        let app = App;
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
        let app = App;
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
        let app = App;
        let mut model = Model::default();
        let account_to_add = Account::new(&AccountType::Apple);

        let _ = app.update(Event::CreateAccount(AccountType::Apple), &mut model, &());
        let does_contain_account = model.accounts.acct.contains(&account_to_add);

        assert!(does_contain_account);
    }

    #[test]
    fn fail_new_platform_account() {
        let app = App;
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
        let app = App;
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
        let app = App;
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
        let app = App;
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
        let app = App;
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
