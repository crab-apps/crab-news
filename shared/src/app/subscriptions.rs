use crate::Error;
use chrono::Local;
use feed_rs::model::Feed;
// use feed_rs::parser::{self, ParseFeedError};
use opml::{self, Head, Outline, OPML};
use serde::{Deserialize, Serialize};

// ANCHOR: type aliases
pub type OpmlFile = String;
pub type OpmlName = String;
pub type FolderName = String;
pub type OldName = String;
pub type OldLink = String;
pub type NewName = String;
pub type OldFolder = Option<FolderName>;
pub type NewFolder = Option<FolderName>;
pub type Subscription = Outline;
pub type SubscriptionTitle = String;
pub type SubscriptionLink = String;
pub type Feeds = Vec<Feed>;
// ANCHOR_END: types aliases

// NOTE - crate: https://crates.io/crates/opml to deal with subscriptions and outlines:
// NOTE - crate: https://crates.io/crates/feed-rs to deal with feeds data *after* subscribtions.
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Subscriptions {
    pub subs: OPML,
    // #[serde(skip)]
    // pub feeds: Feeds,
}

impl Subscriptions {
    // ANCHOR: helper functions
    fn set_test_folder(title: &str) -> Outline {
        Outline {
            text: title.to_string(),
            title: Some(title.to_string()),
            ..Outline::default()
        }
    }

    fn set_test_sub(title: &str, link: &str) -> Outline {
        Outline {
            text: title.to_string(),
            xml_url: Some(link.to_string()),
            ..Outline::default()
        }
    }

    fn set_duplicate_err(action: &str, item: &str, reason: &str) -> self::Error {
        self::Error::AlreadyExists {
            action: action.to_string(),
            item: item.to_string(),
            reason: reason.to_string(),
        }
    }
    // ANCHOR_END: helper functions

    // TODO on duplicates, prompt user for merge or replace
    pub fn import(&self, subs_opml_file: &OpmlFile) -> Result<Self, self::Error> {
        // TODO use proper Shell/WASM functionality to pass on File operations
        let mut file = std::fs::File::open(subs_opml_file).unwrap();
        Ok(Self {
            subs: OPML::from_reader(&mut file)?,
            // feeds: vec![],
        })
    }

    // TODO once shell is implemented, check failures
    pub fn export(&self, subs_opml_name: &OpmlName) -> Result<String, self::Error> {
        let xml_tag = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string();
        let custom_head = Head {
            title: Some(subs_opml_name.to_string()),
            date_created: Some(Local::now().format("%Y - %a %b %e %T").to_string()),
            owner_name: Some("Crab News".to_string()),
            owner_id: Some("https://github.com/crab-apps/crab-news".to_string()),
            ..Head::default()
        };
        let custom_opml = OPML {
            head: Some(custom_head),
            body: self.subs.body.clone(),
            ..OPML::default()
        };
        let export_content = xml_tag + &custom_opml.to_string().unwrap();
        // TODO use proper Shell/WASM functionality to pass on File operations
        match std::fs::write(subs_opml_name, &export_content) {
            Ok(_) => Ok("Subscriptions successfully exported".to_string()),
            Err(e) => Err(Error::Io(e)),
        }
    }

    // NOTE folders are only allowed at root level. no nesting.
    pub fn add_folder(&self, folder_name: &FolderName) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let test_folder = Self::set_test_folder(folder_name);
        let duplicate_err = Self::set_duplicate_err(
            "Cannot add new folder",
            folder_name.as_str(),
            "It already exists.",
        );

        if subs.subs.body.outlines.contains(&test_folder) {
            Err(duplicate_err)
        } else {
            subs.subs.body.outlines.push(test_folder);
            Ok(subs)
        }
    }

    // NOTE folders are only allowed at root level. no nesting.
    pub fn delete_folder(&self, folder_name: &FolderName) -> Self {
        let mut subs = self.clone();
        subs.subs
            .body
            .outlines
            .retain(|name| name.text != *folder_name);
        subs
    }

    // NOTE folders are only allowed at root level. no nesting.
    pub fn rename_folder(
        &self,
        old_folder_name: &OldName,
        new_folder_name: &NewName,
    ) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let test_folder = Self::set_test_folder(new_folder_name);
        let duplicate_err = Self::set_duplicate_err(
            "Cannot rename folder to",
            new_folder_name.as_str(),
            "It already exists.",
        );

        if subs.subs.body.outlines.contains(&test_folder) {
            Err(duplicate_err)
        } else {
            subs.subs
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == *old_folder_name)
                .for_each(|folder| {
                    folder.text = new_folder_name.to_string();
                    folder.title = Some(new_folder_name.to_string());
                });
            Ok(subs)
        }
    }

    // NOTE adding a duplicate sub should always fail no matter where it exists
    pub fn add_subscription(
        &self,
        folder_name: &Option<FolderName>,
        sub_title: &SubscriptionTitle,
        sub_link: &SubscriptionLink,
    ) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let test_subscription = Self::set_test_sub(sub_title, sub_link);
        let duplicate_err = Self::set_duplicate_err(
            "Cannot add new subscription",
            sub_title.as_str(),
            "You are already subscribed.",
        );

        if let Some(folder_text) = &folder_name {
            enum SubStatus {
                AlreadySubscribed,
                AddedNewSub,
            }

            let mut sub_status = SubStatus::AddedNewSub;
            subs.subs
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == *folder_text)
                .for_each(|folder| {
                    if folder.outlines.contains(&test_subscription) {
                        sub_status = SubStatus::AlreadySubscribed;
                    } else {
                        folder.add_feed(sub_title.as_str(), sub_link.as_str());
                    }
                });

            // NOTE I'd rather do this in for_each but closure has no return
            match sub_status {
                SubStatus::AlreadySubscribed => return Err(duplicate_err),
                SubStatus::AddedNewSub => return Ok(subs),
            }
        }

        if subs.subs.body.outlines.contains(&test_subscription) {
            Err(duplicate_err)
        } else {
            subs.subs.add_feed(sub_title.as_str(), sub_link.as_str());
            Ok(subs)
        }
    }

    pub fn delete_subscription(
        &self,
        folder_name: &Option<FolderName>,
        sub_title: &SubscriptionTitle,
    ) -> Self {
        let mut subs = self.clone();
        if let Some(folder_text) = folder_name {
            subs.subs
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == *folder_text)
                .for_each(|folder| folder.outlines.retain(|name| name.text != *sub_title));
        } else {
            subs.subs
                .body
                .outlines
                .retain(|name| name.text != *sub_title);
        }
        subs
    }

    // NOTE rename to an existing sub should always fail no matter where it exists
    pub fn rename_subscription(
        &self,
        folder_name: &Option<FolderName>,
        old_name: &OldName,
        old_link: &OldLink,
        new_name: &NewName,
    ) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let test_subscription = Self::set_test_sub(new_name, old_link);
        let duplicate_err = Self::set_duplicate_err(
            "Cannot rename subscription to",
            new_name.as_str(),
            "It already exists.",
        );

        if let Some(folder_text) = &folder_name {
            enum SubStatus {
                AlreadyExists,
                Renamed,
            }

            let mut sub_status = SubStatus::Renamed;
            subs.subs
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == *folder_text)
                .for_each(|folder| {
                    if folder.outlines.contains(&test_subscription) {
                        sub_status = SubStatus::AlreadyExists;
                    } else {
                        folder
                            .outlines
                            .iter_mut()
                            .filter(|sub| sub.text == *old_name)
                            .for_each(|sub| {
                                sub.text = new_name.to_string();
                            });
                    }
                });

            // NOTE I'd rather do this in for_each but closure has no return
            match sub_status {
                SubStatus::AlreadyExists => return Err(duplicate_err),
                SubStatus::Renamed => return Ok(subs),
            }
        }

        if subs.subs.body.outlines.contains(&test_subscription) {
            Err(duplicate_err)
        } else {
            subs.subs
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == *old_name)
                .for_each(|sub| sub.text = new_name.to_string());
            Ok(subs)
        }
    }

    pub fn move_subscription(
        &self,
        subscription: &Subscription,
        old_folder: &OldFolder,
        new_folder: &NewFolder,
    ) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let duplicate_err = Self::set_duplicate_err(
            "Cannot move subscription to",
            subscription.text.as_str(),
            "It already exists.",
        );
        match (old_folder, new_folder) {
            (None, Some(folder_new)) => {
                subs = Self::delete_subscription(&subs, &None, &subscription.text.to_string());
                match Self::add_subscription(
                    &subs,
                    &Some(folder_new.to_string()),
                    &subscription.text.to_string(),
                    &subscription.xml_url.clone().unwrap(),
                ) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(duplicate_err),
                }
            }
            (Some(folder_old), None) => {
                subs = Self::delete_subscription(
                    &subs,
                    &Some(folder_old.to_string()),
                    &subscription.text.to_string(),
                );
                match Self::add_subscription(
                    &subs,
                    &None,
                    &subscription.text.to_string(),
                    &subscription.xml_url.clone().unwrap(),
                ) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(duplicate_err),
                }
            }
            (Some(folder_old), Some(folder_new)) => {
                subs = Self::delete_subscription(
                    &subs,
                    &Some(folder_old.to_string()),
                    &subscription.text.to_string(),
                );
                match Self::add_subscription(
                    &subs,
                    &Some(folder_new.to_string()),
                    &subscription.text.to_string(),
                    &subscription.xml_url.clone().unwrap(),
                ) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(duplicate_err),
                }
            }
            (None, None) => Err(duplicate_err),
        }
    }

    // pub fn add_feed(&self, body: Vec<u8>) -> Result<Self, ParseFeedError> {
    //     let mut subs = self.clone();
    //     let feed = parser::parse(&*body)?;
    //     subs.feeds.push(feed);
    //     Ok(subs)
    // }

    // pub fn find_feed(&self, sub_title: &SubscriptionTitle) -> Feed {
    //     self.feeds
    //         .iter()
    //         .find(|feed| feed.title.clone().unwrap().content == *sub_title)
    //         .unwrap()
    //         .clone()
    // }
}

#[cfg(test)]
mod import_export {
    use super::*;
    use crate::{Account, AccountType, Accounts, AccountsExt};
    use crate::{CrabNews, Event, Model};
    use chrono::prelude::Local;
    use opml::OPML;

    #[test]
    fn import_subscriptions() {
        let app = CrabNews;
        let mut model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let subs_opml_file = "example_import.opml".to_string();
        let example_subs = r#"<opml version="2.0"><head><title>Subscriptions.opml</title><dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated><ownerName>Crab News</ownerName></head><body><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/><outline text="Group Name" title="Group Name"><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/></outline></body></opml>"#;

        let _ = app.update(
            Event::ImportSubscriptions(account.clone(), subs_opml_file),
            &mut model,
        );
        let added_subs = model.accounts[acct_index].subs.clone();
        let expected_subs = Subscriptions {
            subs: OPML::from_str(example_subs).unwrap(),
            // feeds: vec![],
        };

        assert_eq!(added_subs, expected_subs);
    }

    #[test]
    fn fail_import_for_invalid_xml() {
        let app = CrabNews;
        let mut model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let subs_opml_file = "invalid_xml.opml".to_string();

        let _ = app.update(
            Event::ImportSubscriptions(account, subs_opml_file),
            &mut model,
        );
        let actual_error = model.notification.message;
        let expected_error = "Failed to process XML file";

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn fail_import_for_invalid_opml_version() {
        let app = CrabNews;
        let mut model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let subs_opml_file = "invalid_opml_version.opml".to_string();

        let _ = app.update(
            Event::ImportSubscriptions(account, subs_opml_file),
            &mut model,
        );
        let actual_error = model.notification.message;
        let expected_error = "Unsupported OPML version: \"0.1\"";

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn fail_import_for_body_has_no_outlines() {
        let app = CrabNews;
        let mut model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let subs_opml_file = "invalid_body.opml".to_string();

        let _ = app.update(
            Event::ImportSubscriptions(account, subs_opml_file),
            &mut model,
        );
        let actual_error = model.notification.message;
        let expected_error = "OPML body has no <outline> elements";

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn export_subscriptions() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let date_created = Some(Local::now().format("%Y - %a %b %e %T").to_string());
        let subs_opml_name = "Subscriptions.opml".to_string();

        #[allow(clippy::unnecessary_literal_unwrap)]
        let example_subs = format!("<opml version=\"2.0\"><head><title>{}</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName><ownerId>https://github.com/crab-apps/crab-news</ownerId></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", subs_opml_name, date_created.unwrap());

        model.accounts[acct_index].subs = Subscriptions {
            subs: OPML::from_str(&example_subs).unwrap(),
            // feeds: vec![],
        };
        let imported_content = model.accounts[acct_index].subs.clone();

        let _ = app.update(
            Event::ExportSubscriptions(account, subs_opml_name.to_string()),
            &mut model,
        );

        // TODO use proper Shell/WASM/crate functionality to File operations
        let mut exported_file = std::fs::File::open(subs_opml_name).unwrap();
        let exported_content = Subscriptions {
            subs: OPML::from_reader(&mut exported_file).unwrap(),
            // feeds: vec![],
        };

        assert_eq!(exported_content, imported_content);
    }

    #[test]
    fn export_subscriptions_notification() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let date_created = Some(Local::now().format("%Y - %a %b %e %T").to_string());
        let subs_opml_name = "Subscriptions.opml".to_string();

        #[allow(clippy::unnecessary_literal_unwrap)]
        let example_subs = format!("<opml version=\"2.0\"><head><title>{}</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName><ownerId>https://github.com/crab-apps/crab-news</ownerId></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", subs_opml_name, date_created.unwrap());

        model.accounts[acct_index].subs = Subscriptions {
            subs: OPML::from_str(&example_subs).unwrap(),
            // feeds: vec![],
        };

        let _ = app.update(
            Event::ExportSubscriptions(account, subs_opml_name.to_string()),
            &mut model,
        );

        let actual_error = model.notification.message;
        let expected_error = "Subscriptions successfully exported";

        assert_eq!(actual_error, expected_error);
    }

    // TODO once shell is implemented, check failures
    // #[test]
    // fn fail_export_subscriptions() {
    //     let app = CrabNews;
    //     let mut model: Model = Model::default();
    //     let date_created = Some(Local::now().format("%Y - %a %b %e %T").to_string());
    //     let subs_opml_name = format!("{} - Subscriptions.opml", date_created.clone().unwrap());
    //     let example_subs = format!("<opml version=\"2.0\"><head><title>{}</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName><ownerId>https://github.com/crab-apps/crab-news</ownerId></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", subs_opml_name, date_created.unwrap());

    //     model.subscriptions = Subscriptions {
    //         opml: OPML::from_str(&example_subs).unwrap(),
    //     };
    //     let imported_content = model.subscriptions.clone();

    //     let _ = app.update(
    //         Event::ExportSubscriptions(subs_opml_name.clone()),
    //         &mut model,
    //     );

    //     // TODO use proper Shell/WASM/crate functionality to File operations
    //     let mut exported_file = std::fs::File::open(subs_opml_name.clone()).unwrap();
    //     let exported_content = Subscriptions {
    //         opml: OPML::from_reader(&mut exported_file).unwrap(),
    //     };

    //     assert_eq!(exported_content, imported_content);
}

#[cfg(test)]
mod folder {
    use super::*;
    use crate::{Account, AccountType, Accounts, AccountsExt};
    use crate::{CrabNews, Event, Model};
    use opml::Outline;

    #[test]
    fn add_new_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let folder_name = "Added Folder".to_string();
        let added_folder = &Outline {
            text: folder_name.to_string(),
            title: Some(folder_name.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account, folder_name.to_string()),
            &mut model,
        );
        let does_contain_folder = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .contains(added_folder);

        assert!(does_contain_folder);
    }

    #[test]
    fn add_two_new_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let folder_name_one = "Added Folder Ome".to_string();
        let folder_name_two = "Added Folder Two".to_string();
        let added_folder_one = &Outline {
            text: folder_name_one.to_string(),
            title: Some(folder_name_one.to_string()),
            ..Outline::default()
        };
        let added_folder_two = &Outline {
            text: folder_name_two.to_string(),
            title: Some(folder_name_two.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name_one.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddNewFolder(account, folder_name_two.to_string()),
            &mut model,
        );
        let does_contain_folder_one = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .contains(added_folder_one);
        let does_contain_folder_two = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .contains(added_folder_two);

        assert!((does_contain_folder_one && does_contain_folder_two));
    }

    #[test]
    fn fail_add_new_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let folder_name = "Added Folder".to_string();

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot add new folder \"{}\". It already exists.",
            folder_name
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn delete_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let deleted_folder = &Outline {
            text: "Deleted Folder".to_string(),
            title: Some("Deleted Folder".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), deleted_folder.text.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::DeleteFolder(account.clone(), deleted_folder.text.to_string()),
            &mut model,
        );

        let does_contain_folder = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .contains(deleted_folder);

        assert!(!does_contain_folder);
    }

    #[test]
    fn rename_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let rename_folder = &Outline {
            text: "Rename Folder".to_string(),
            title: Some("Rename Folder".to_string()),
            ..Outline::default()
        };

        let expected_folder = &Outline {
            text: "Expected Folder".to_string(),
            title: Some("Expected Folder".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), rename_folder.text.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::RenameFolder(
                account.clone(),
                rename_folder.text.to_string(),
                expected_folder.text.to_string(),
            ),
            &mut model,
        );

        let does_contain_folder = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .contains(expected_folder);

        assert!(does_contain_folder);
    }

    #[test]
    fn fail_rename_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let test_folder = &Outline {
            text: "Expected Folder".to_string(),
            title: Some("Expected Folder".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), test_folder.text.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::RenameFolder(
                account.clone(),
                test_folder.text.to_string(),
                test_folder.text.to_string(),
            ),
            &mut model,
        );
        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot rename folder to \"{}\". It already exists.",
            test_folder.text
        );

        assert_eq!(actual_error, expected_error);
    }
}

// TODO add: title, description, type, version, html_url. are these derived or manual?
#[cfg(test)]
mod add_subscription {
    use super::*;
    use crate::{Account, AccountType, Accounts, AccountsExt};
    use crate::{CrabNews, Event, Model};
    use opml::Outline;

    #[test]
    fn add_new_subscription_to_root() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let sub_title = "New Sub Root".to_string();
        let sub_link = "https://example.com/atom.xml".to_string();
        let expected_sub = &Outline {
            text: sub_title.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                sub_title.to_string(),
                sub_link.to_string(),
            ),
            &mut model,
        );

        let does_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .contains(expected_sub);

        assert!(does_contain_sub);
    }

    #[test]
    fn add_new_subscription_to_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let folder_name = "New Sub Folder".to_string();
        let sub_title = "New Sub Folder".to_string();
        let sub_link = "https://example.com/atom.xml".to_string();
        let expected_sub = &Outline {
            text: sub_title.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                sub_title.to_string(),
                sub_link.to_string(),
            ),
            &mut model,
        );

        #[allow(clippy::unnecessary_find_map)]
        let does_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert!(does_contain_sub);
    }

    #[test]
    fn fail_add_new_subscription_to_root() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let sub_title = "New Sub Root".to_string();
        let sub_link = "https://example.com/atom.xml".to_string();
        let test_subscription = &Outline {
            text: sub_title.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                sub_title.to_string(),
                sub_link.to_string(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                sub_title.to_string(),
                sub_link.to_string(),
            ),
            &mut model,
        );
        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot add new subscription \"{}\". You are already subscribed.",
            test_subscription.text
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn fail_add_new_subscription_to_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let folder_name = "New Sub Folder".to_string();
        let sub_title = "New Sub Folder".to_string();
        let sub_link = "https://example.com/atom.xml".to_string();
        let test_subscription = &Outline {
            text: sub_title.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                sub_title.to_string(),
                sub_link.to_string(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                sub_title.to_string(),
                sub_link.to_string(),
            ),
            &mut model,
        );
        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot add new subscription \"{}\". You are already subscribed.",
            test_subscription.text
        );

        assert_eq!(actual_error, expected_error);
    }
}

#[cfg(test)]
mod delete_subscription {
    use super::*;
    use crate::{Account, AccountType, Accounts, AccountsExt};
    use crate::{CrabNews, Event, Model};
    use opml::Outline;

    #[test]
    fn delete_subscription_from_root() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let deleted_sub = &Outline {
            text: "Deleted Sub Root".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                deleted_sub.text.to_string(),
                deleted_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::DeleteSubscription(account.clone(), None, deleted_sub.text.to_string()),
            &mut model,
        );

        let does_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .contains(deleted_sub);

        assert!(!does_contain_sub);
    }

    #[test]
    fn delete_subscription_from_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let folder_name = "Deleted Sub Folder".to_string();
        let deleted_sub = &Outline {
            text: "Sub Name".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                deleted_sub.text.to_string(),
                deleted_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::DeleteSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                deleted_sub.text.to_string(),
            ),
            &mut model,
        );

        #[allow(clippy::unnecessary_find_map)]
        let does_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(deleted_sub)))
            .unwrap();

        assert!(!does_contain_sub);
    }

    #[test]
    fn delete_subscription_from_folder_with_multi_subs() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let folder_name = "Deleted Multi Subs".to_string();
        let delete_sub = &Outline {
            text: "Deleted Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };
        let expected_sub = &Outline {
            text: "Expected Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                delete_sub.text.to_string(),
                delete_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::DeleteSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                delete_sub.text.to_string(),
            ),
            &mut model,
        );

        #[allow(clippy::unnecessary_find_map)]
        let does_contain_deleted_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(delete_sub)))
            .unwrap();

        #[allow(clippy::unnecessary_find_map)]
        let does_contain_expected_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert!((!does_contain_deleted_sub && does_contain_expected_sub));
    }
}

#[cfg(test)]
mod rename_subscription {
    use super::*;
    use crate::{Account, AccountType, Accounts, AccountsExt};
    use crate::{CrabNews, Event, Model};
    use opml::Outline;

    #[test]
    fn rename_subscription_in_root() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };
        let expected_sub = &Outline {
            text: "Renamed Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                None,
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
                expected_sub.text.to_string(),
            ),
            &mut model,
        );

        let does_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .contains(expected_sub);

        assert!(does_contain_sub);
    }

    #[test]
    fn fail_rename_subscription_in_root() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                None,
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
                rename_sub.text.to_string(),
            ),
            &mut model,
        );

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot rename subscription to \"{}\". It already exists.",
            rename_sub.text
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn rename_subscription_in_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let folder_name = "Renamed Sub Folder".to_string();
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };
        let expected_sub = &Outline {
            text: "Renamed Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
                expected_sub.text.to_string(),
            ),
            &mut model,
        );

        #[allow(clippy::unnecessary_find_map)]
        let does_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert!(does_contain_sub);
    }

    #[test]
    fn fail_rename_subscription_in_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let folder_name = "Renamed Sub Folder".to_string();
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
                rename_sub.text.to_string(),
            ),
            &mut model,
        );

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot rename subscription to \"{}\". It already exists.",
            rename_sub.text
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn rename_subscription_in_folder_with_multi_subs() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let folder_name = "Renamed Multi Sub Folder".to_string();
        let untouched_sub = &Outline {
            text: "Untouched Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };
        let expected_sub = &Outline {
            text: "Renamed Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                untouched_sub.text.to_string(),
                untouched_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
                expected_sub.text.to_string(),
            ),
            &mut model,
        );

        #[allow(clippy::unnecessary_find_map)]
        let does_contain_untouched_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(untouched_sub)))
            .unwrap();

        #[allow(clippy::unnecessary_find_map)]
        let does_contain_expected_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert!((does_contain_untouched_sub && does_contain_expected_sub));
    }

    #[test]
    fn fail_rename_subscription_in_folder_with_multi_subs() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let folder_name = "Renamed Multi Sub Folder".to_string();
        let untouched_sub = &Outline {
            text: "Untouched Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                untouched_sub.text.to_string(),
                untouched_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                rename_sub.text.to_string(),
                rename_sub.xml_url.clone().unwrap().clone(),
                rename_sub.text.to_string(),
            ),
            &mut model,
        );

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot rename subscription to \"{}\". It already exists.",
            rename_sub.text
        );

        assert_eq!(actual_error, expected_error);
    }
}

#[cfg(test)]
mod move_subscription {
    use super::*;
    use crate::{Account, AccountType, Accounts, AccountsExt};
    use crate::{CrabNews, Event, Model};
    use opml::Outline;

    #[test]
    fn move_subscription_from_root_to_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let folder_name = "Move Sub To Folder".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                None,
                Some(folder_name.to_string()),
            ),
            &mut model,
        );

        let does_root_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .contains(expected_sub);

        #[allow(clippy::unnecessary_find_map)]
        let does_folder_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert!((!does_root_contain_sub && does_folder_contain_sub));
    }

    #[test]
    fn fail_move_subscription_from_root_to_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let folder_name = "Move Sub To Folder".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                None,
                Some(folder_name.to_string()),
            ),
            &mut model,
        );

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot move subscription to \"{}\". It already exists.",
            expected_sub.text
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn move_subscription_from_folder_to_root() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let folder_name = "Move Sub To Root".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                Some(folder_name.to_string()),
                None,
            ),
            &mut model,
        );

        let does_root_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .contains(expected_sub);

        #[allow(clippy::unnecessary_find_map)]
        let does_folder_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert!((does_root_contain_sub && !does_folder_contain_sub));
    }

    #[test]
    fn fail_move_subscription_from_folder_to_root() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let folder_name = "Move Sub To Root".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.to_string()),
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                Some(folder_name.to_string()),
                None,
            ),
            &mut model,
        );

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot move subscription to \"{}\". It already exists.",
            expected_sub.text
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn move_subscription_from_folder_to_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let acct_index = Accounts::find_account_index(&model.accounts, &account);
        let folder_one = "Folder One".to_string();
        let folder_two = "Folder Two".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_one.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_two.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_one.to_string()),
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                Some(folder_one.to_string()),
                Some(folder_two.to_string()),
            ),
            &mut model,
        );

        #[allow(clippy::unnecessary_find_map)]
        let does_folder_one_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_one)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        #[allow(clippy::unnecessary_find_map)]
        let does_folder_two_contain_sub = model.accounts[acct_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_two)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert!((!does_folder_one_contain_sub && does_folder_two_contain_sub));
    }

    #[test]
    fn fail_move_subscription_from_folder_to_folder() {
        let app = CrabNews;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
        let folder_one = "Folder One".to_string();
        let folder_two = "Folder Two".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_one.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_two.to_string()),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_one.to_string()),
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_two.to_string()),
                expected_sub.text.to_string(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                Some(folder_one.to_string()),
                Some(folder_two.to_string()),
            ),
            &mut model,
        );

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot move subscription to \"{}\". It already exists.",
            expected_sub.text
        );

        assert_eq!(actual_error, expected_error);
    }
}

#[cfg(test)]
mod feeds {
    // use super::*;
    // use crate::{Account, AccountType, Accounts, AccountsExt};
    // use crate::{CrabNews, Effect, Event, Model};

    // mode shell START
    // use anyhow::Result;
    // use crux_core::Core;
    // use crux_http::protocol::{HttpRequest, HttpResponse, HttpResult};
    // use std::collections::VecDeque;

    // #[allow(clippy::large_enum_variant)]
    // enum Task {
    //     Event(Event),
    //     Effect(Effect),
    // }

    // pub(crate) fn run(core: &Core<CrabNews>, event: Event) -> Result<Vec<HttpRequest>> {
    //     let mut queue: VecDeque<Task> = VecDeque::new();

    //     queue.push_back(Task::Event(event));

    //     let mut received: Vec<HttpRequest> = vec![];

    //     while !queue.is_empty() {
    //         let task = queue.pop_front().expect("an event");

    //         match task {
    //             Task::Event(event) => {
    //                 enqueue_effects(&mut queue, core.process_event(event));
    //             }
    //             Task::Effect(effect) => match effect {
    //                 Effect::Render(_) => (),
    //                 Effect::Http(mut request) => {
    //                     let http_request = &request.operation;

    //                     received.push(http_request.clone());
    //                     let response = HttpResponse::ok().json("Hello").build();

    //                     enqueue_effects(
    //                         &mut queue,
    //                         core.resolve(&mut request, HttpResult::Ok(response))
    //                             .expect("effect should resolve"),
    //                     );
    //                 }
    //             },
    //         };
    //     }

    //     Ok(received)
    // }

    // fn enqueue_effects(queue: &mut VecDeque<Task>, effects: Vec<Effect>) {
    //     queue.append(&mut effects.into_iter().map(Task::Effect).collect())
    // }
    // mod shell END

    // #[test]
    // fn get_feed() -> Result<(), Box<dyn std::error::Error>> {
    //     let app = CrabNews;
    //     let mut model: Model = Model::default();
    //     let sub_title = "Gentle Wash Records".to_string();
    //     let sub_link = "https://gentlewashrecords.com/atom.xml".to_string();

    //     let account = Account::new(&AccountType::Local);
    //     let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
    //     let acct_index = Accounts::find_account_index(&model.accounts, &account);

    //     let _ = app.update(
    //         Event::AddSubscription(account.clone(), None, sub_title, sub_link.to_string()),
    //         &mut model,
    //     );

    //     let core: Core<CrabNews> = Core::default();

    //     let received = run(&core, Event::GetFeed(account, sub_link.to_string()))?;

    //     assert_eq!(received, vec![HttpRequest::get(sub_link).build()]);
    //     Ok(())
    // }

    // #[test]
    // fn add_new_feed() {
    //     let app = CrabNews;
    //     let mut model: Model = Model::default();

    //     let account = Account::new(&AccountType::Local);
    //     let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
    //     let acct_index = Accounts::find_account_index(&model.accounts, &account);

    //     let sub_title = "Gentle Wash Records".to_string();
    //     let example_rss = r#"<?xml version="1.0" encoding="UTF-8" ?>
    //       <rss version="2.0">
    //         <channel>
    //           <title>Gentle Wash Records</title>
    //           <description>This is an example of an RSS feed</description>
    //           <link>http://www.example.com/main.html</link>
    //           <lastBuildDate>Mon, 06 Sep 2010 00:01:00 +0000</lastBuildDate>
    //           <pubDate>Sun, 06 Sep 2009 16:20:00 +0000</pubDate>
    //           <ttl>1800</ttl>

    //           <item>
    //             <title>Example entry</title>
    //             <description>Here is some text containing an interesting description.</description>
    //             <link>http://www.example.com/blog/post/1</link>
    //             <guid isPermaLink="true">7bd204c6-1655-4c27-aeee-53f933c5395f</guid>
    //             <pubDate>Sun, 06 Sep 2009 16:20:00 +0000</pubDate>
    //           </item>

    //         </channel>
    //       </rss>"#;

    //     let body = Vec::from(example_rss.as_bytes());
    //     let _ = app.update(Event::SetFeed(account, example_rss.as_bytes()), &mut model);
    //     let added_feed = Subscriptions::find_feed(&model.accounts[acct_index].subs, &sub_title);

    //     assert_eq!(added_feed.title.unwrap().content, sub_title);
    // }

    // #[test]
    // fn fail_add_feed_already_exists() {
    //     let app = CrabNews;
    //     let mut model: Model = Model::default();
    //     let account = Account::new(&AccountType::Local);
    //     let acct_index = Accounts::find_account_index(&model.accounts, &account);
    //     let sub_title = "Gentle Wash Records".to_string();
    //     let sub_link = "https://gentlewashrecords.com/atom.xml".to_string();

    //     let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model);
    //     let _ = app.update(
    //         Event::AddNewSubscription(
    //             account.clone(),
    //             None,
    //             sub_title.to_string(),
    //             sub_link.to_string(),
    //         ),
    //         &mut model,
    //     );
    //     let _response = app
    //         .update(Event::GetFeed(account.clone(), sub_link), &mut model)
    //         .expect_one_event();
    //     // let _ = app.update(Event::SetFeed(account.clone(), response), &mut model);

    //     let added_feed = Subscriptions::find_feed(&model.accounts[acct_index].subs, &sub_title);
    //     let added_feed_title = added_feed.title.clone().unwrap().content;
    //     let actual_error = model.notification.message;
    //     let expected_error = format!(
    //         "Cannot add new subscription \"{}\". You are already subscribed.",
    //         added_feed_title
    //     );

    //     assert_eq!(actual_error, expected_error);
    // }
}
