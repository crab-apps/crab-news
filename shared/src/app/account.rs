use super::subscriptions::Subscriptions;
use opml::OPML;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{action} \"{account}\". {reason}")]
    AccountAlreadyExists {
        action: String,
        account: String,
        reason: String,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub subs: Subscriptions,
}

// TODO use crux_platform instead
#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Clone)]
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
    fn set_account_name(account_type: &AccountType) -> String {
        match account_type {
            AccountType::Local => "On Device".to_string(),
            AccountType::Apple => "iCloud".to_string(),
            AccountType::Google => "Google Sync".to_string(),
            AccountType::Microsoft => "Live 365".to_string(),
            AccountType::Canonical => "Ubuntu One".to_string(),
        }
    }

    fn set_duplicate_err(action: &str, account: &str, reason: &str) -> self::Error {
        self::Error::AccountAlreadyExists {
            action: action.to_string(),
            account: account.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn new(account_type: &AccountType) -> Self {
        Account {
            id: Uuid::new_v4(),
            name: Self::set_account_name(&account_type),
            subs: Subscriptions {
                opml: OPML::default(),
            },
        }
    }

    pub fn add_account(
        accounts: &Vec<Account>,
        account_type: AccountType,
    ) -> Result<Vec<Account>, self::Error> {
        let mut accounts = accounts.clone();
        let account_to_add = Self::new(&account_type);
        let account_name = Self::set_account_name(&account_type);
        let duplicate_err = Self::set_duplicate_err(
            "Cannot add account",
            account_name.as_str(),
            "It already exists.",
        );

        if accounts.contains(&account_to_add) {
            Err(duplicate_err)
        } else {
            accounts.push(account_to_add);
            Ok(accounts)
        }
    }

    pub fn delete(accounts: &Vec<Account>, account_type: AccountType) -> Vec<Account> {
        let mut accounts = accounts.clone();
        let account_to_delete = Self::set_account_name(&account_type);

        accounts.retain(|account| account.name != account_to_delete);
        accounts
    }
}
