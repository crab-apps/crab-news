use super::subscriptions::Subscriptions;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type AccountName = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Account {
    id: Uuid,
    _type: AccountType,
    subs: Subscriptions,
}

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountType {
    Local(AccountLocal),
    Native(AccountNative),
    Cloud(AccountCloud),
}

#[derive(Default, Debug, Hash, Serialize, Deserialize, PartialEq, Clone)]
pub enum AuthStatus {
    Authehticated,
    #[default]
    LoggedOff,
}

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountLocal {
    Local { name: String, auth: AuthStatus },
}

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountNative {
    // how do I check for Auth? impl? Capabilities?
    Apple { name: String, auth: AuthStatus },
    Google { name: String, auth: AuthStatus },
    Microsoft { name: String, auth: AuthStatus },
    Canonical { name: String, auth: AuthStatus },
    // more?
}

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountCloud {
    // https://rclone.org
    Dropbox { name: String, auth: AuthStatus },
    // more
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CrabNews, Event, Model};
    use crux_core::testing::AppTester;

    #[test]
    fn add_new_local_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
    }

    #[test]
    fn fail_local_account_exists() {}

    #[test]
    fn add_new_native_account() {}

    #[test]
    fn fail_native_account_exists() {}

    // NOTE can't add other platforns native accts
    // i.e: Apple:iCloud | Android:Google | Microsoft:Live | Canonical:One only
    #[test]
    fn fail_native_account_type() {}

    #[test]
    fn add_new_cloud_account() {}

    #[test]
    fn fail_cloud_account_exists() {}

    #[test]
    fn delete_local_account() {}

    #[test]
    fn delete_native_account() {}

    #[test]
    fn delete_cloud_account() {}
}
