use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountType {
    Local(AccountLocal),
    Native(AccountNative),
    Cloud(AccountCloud),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountLocal {
    Local { name: String, auth: bool },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountNative {
    // how do I check for Auth? impl? Capabilities?
    Apple { name: String, auth: bool },
    Google { name: String, auth: bool },
    Microsoft { name: String, auth: bool },
    Canonical { name: String, auth: bool },
    // more?
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountCloud {
    // https://rclone.org
    Dropbox { name: String, auth: bool },
    // more
}
