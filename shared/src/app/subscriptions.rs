use super::Error;
use super::Feeds;
use crate::define_newtype;

use chrono::Local;
use feed_rs::model::Feed;
use opml::{Head, Outline, OPML};
use serde::{Deserialize, Serialize};

// ANCHOR: types
// Generate new types using the macro
define_newtype!(OpmlName);
define_newtype!(FolderName);
define_newtype!(OldFolderName);
define_newtype!(NewFolderName);
define_newtype!(SubscriptionTitle);
define_newtype!(SubscriptionLink);
define_newtype!(OldSubscriptionName);
define_newtype!(NewSubscriptionName);

// Optional types and type aliases remain the same
pub type OldFolder = Option<FolderName>;
pub type NewFolder = Option<FolderName>;
pub type Subscription = Outline;
// ANCHOR_END: types

// NOTE - crate: https://crates.io/crates/opml to deal with subscriptions and outlines.
// NOTE - crate: https://crates.io/crates/feed-rs to deal with feeds data *after* subscriptions.
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Subscriptions {
    pub feeds: Feeds,
    pub subs: OPML,
}

trait SubscriptionHelpers {
    fn set_test_folder(title: &str) -> Outline;
    fn set_test_sub(title: &str, sub_link: &str) -> Outline;
}

impl SubscriptionHelpers for Subscriptions {
    fn set_test_folder(title: &str) -> Outline {
        Outline {
            text: title.to_string(),
            title: Some(title.to_string()),
            ..Outline::default()
        }
    }

    fn set_test_sub(title: &str, sub_link: &str) -> Outline {
        Outline {
            text: title.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        }
    }
}

trait ImportSubscriptions {
    fn import_subscriptions(&self, opml_file_content: &str) -> Result<Self, Error>
    where
        Self: Sized;
}

// TODO on duplicates, prompt user for merge or replace
// FIXME refactor this to drive the adapter to store data in the database
impl ImportSubscriptions for Subscriptions {
    fn import_subscriptions(&self, opml_file_content: &str) -> Result<Self, Error> {
        let subs = self.clone();

        Ok(Self {
            subs: OPML::from_str(opml_file_content)?,
            feeds: subs.feeds,
        })
    }
}

trait ExportSubscriptions {
    fn export_subscriptions(&self, subs_opml_name: &OpmlName) -> Result<String, Error>;
}

impl ExportSubscriptions for Subscriptions {
    // TODO once shell is implemented, check failures
    fn export_subscriptions(&self, subs_opml_name: &OpmlName) -> Result<String, Error> {
        let xml_tag = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string();
        let custom_head = Head {
            title: Some(subs_opml_name.to_string()),
            date_created: Some(Local::now().format("%Y - %a %b %e %T").to_string()),
            owner_name: Some("Crab News".to_string()),
            owner_id: Some("https://github.com/crab-apps/crab-news".to_string()),
            ..Head::default()
        };
        let custom_opml = OPML {
            version: "2.0".to_string(),
            head: Some(custom_head),
            body: self.subs.body.clone(),
        };
        let export_content = xml_tag + &custom_opml.to_string().unwrap();
        // TODO use proper Shell/WASM functionality to pass on File operations
        match std::fs::write(subs_opml_name.to_string(), &export_content) {
            Ok(_) => Ok("Subscriptions successfully exported".to_string()),
            Err(e) => Err(Error::Io(e)),
        }
    }
}

trait AddFolder {
    fn add_folder(&self, folder_name: &FolderName) -> Result<Self, Error>
    where
        Self: Sized;
}

impl AddFolder for Subscriptions {
    // NOTE folders are only allowed at root level. no nesting.
    fn add_folder(&self, folder_name: &FolderName) -> Result<Self, Error> {
        let mut subs = self.clone();
        let test_folder = Self::set_test_folder(folder_name.as_ref());
        let duplicate_err = Error::set_error(
            "Cannot add new folder",
            folder_name.0.as_str(),
            "It already exists.",
        );

        if subs.subs.body.outlines.contains(&test_folder) {
            Err(duplicate_err)
        } else {
            subs.subs.body.outlines.push(test_folder);
            Ok(subs)
        }
    }
}

trait DeleteFolder {
    fn delete_folder(&self, folder_name: &FolderName) -> Self
    where
        Self: Sized;
}

impl DeleteFolder for Subscriptions {
    // NOTE folders are only allowed at root level. no nesting.
    fn delete_folder(&self, folder_name: &FolderName) -> Self {
        let mut subs = self.clone();
        subs.subs
            .body
            .outlines
            .retain(|name| name.text != folder_name.to_string());
        subs
    }
}

trait RenameFolder {
    fn rename_folder(
        &self,
        old_folder_name: &OldFolderName,
        new_folder_name: &NewFolderName,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

impl RenameFolder for Subscriptions {
    // NOTE folders are only allowed at root level. no nesting.
    fn rename_folder(
        &self,
        old_folder_name: &OldFolderName,
        new_folder_name: &NewFolderName,
    ) -> Result<Self, Error> {
        let mut subs = self.clone();
        let test_folder = Self::set_test_folder(new_folder_name.0.as_str());
        let duplicate_err = Error::set_error(
            "Cannot rename folder to",
            new_folder_name.0.as_str(),
            "It already exists.",
        );

        if subs.subs.body.outlines.contains(&test_folder) {
            Err(duplicate_err)
        } else {
            subs.subs
                .body
                .outlines
                .iter_mut()
                .filter(|folder| folder.text == old_folder_name.to_string())
                .for_each(|folder| {
                    folder.text = new_folder_name.to_string();
                    folder.title = Some(new_folder_name.to_string());
                });
            Ok(subs)
        }
    }
}

trait AddSubscription {
    fn add_subscription(
        &self,
        folder_name: &Option<FolderName>,
        sub_title: &SubscriptionTitle,
        sub_link: &SubscriptionLink,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

impl AddSubscription for Subscriptions {
    // NOTE adding a duplicate sub should always fail no matter where it exists
    fn add_subscription(
        &self,
        folder_name: &Option<FolderName>,
        sub_title: &SubscriptionTitle,
        sub_link: &SubscriptionLink,
    ) -> Result<Self, Error> {
        let mut subs = self.clone();
        let test_subscription = Self::set_test_sub(sub_title.0.as_str(), sub_link.0.as_str());
        let duplicate_err = Error::set_error(
            "Cannot add new subscription",
            sub_title.0.as_str(),
            "You are already subscribed.",
        );

        if let Some(folder_text) = &folder_name {
            for folder in subs
                .subs
                .body
                .outlines
                .iter_mut()
                .filter(|folder| folder.text == *folder_text.to_string())
            {
                if folder.outlines.contains(&test_subscription) {
                    return Err(duplicate_err);
                } else {
                    folder.add_feed(sub_title.0.as_str(), sub_link.0.as_str());
                }
            }

            return Ok(subs);
        }

        if subs.subs.body.outlines.contains(&test_subscription) {
            Err(duplicate_err)
        } else {
            subs.subs
                .add_feed(sub_title.0.as_str(), sub_link.0.as_str());
            Ok(subs)
        }
    }
}

trait DeleteSubscription {
    fn delete_subscription(
        &self,
        folder_name: &Option<FolderName>,
        sub_title: &SubscriptionTitle,
    ) -> Self
    where
        Self: Sized;
}

impl DeleteSubscription for Subscriptions {
    fn delete_subscription(
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
                .filter(|folder| folder.text == folder_text.to_string())
                .for_each(|folder| {
                    folder
                        .outlines
                        .retain(|name| name.text != sub_title.to_string())
                });
        } else {
            subs.subs
                .body
                .outlines
                .retain(|name| name.text != sub_title.to_string());
        }
        subs
    }
}

trait RenameSubscription {
    fn rename_subscription(
        &self,
        folder_name: &Option<FolderName>,
        sub_link: &SubscriptionLink,
        old_sub_name: &OldSubscriptionName,
        new_sub_name: &NewSubscriptionName,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

impl RenameSubscription for Subscriptions {
    // NOTE rename to an existing sub should always fail no matter where it exists
    fn rename_subscription(
        &self,
        folder_name: &Option<FolderName>,
        sub_link: &SubscriptionLink,
        old_sub_name: &OldSubscriptionName,
        new_sub_name: &NewSubscriptionName,
    ) -> Result<Self, Error> {
        let mut subs = self.clone();
        let test_subscription = Self::set_test_sub(new_sub_name.0.as_str(), sub_link.0.as_str());
        let duplicate_err = Error::set_error(
            "Cannot rename subscription to",
            new_sub_name.0.as_str(),
            "It already exists.",
        );

        if let Some(folder_text) = &folder_name {
            for folder in subs
                .subs
                .body
                .outlines
                .iter_mut()
                .filter(|folder| folder.text == *folder_text.to_string())
            {
                if folder.outlines.contains(&test_subscription) {
                    return Err(duplicate_err);
                } else {
                    folder
                        .outlines
                        .iter_mut()
                        .filter(|sub| sub.text == old_sub_name.to_string())
                        .for_each(|sub| {
                            sub.text = new_sub_name.to_string();
                        });
                }
            }

            return Ok(subs);
        }

        if subs.subs.body.outlines.contains(&test_subscription) {
            Err(duplicate_err)
        } else {
            subs.subs
                .body
                .outlines
                .iter_mut()
                .filter(|sub| sub.text == old_sub_name.to_string())
                .for_each(|sub| sub.text = new_sub_name.to_string());
            Ok(subs)
        }
    }
}

trait MoveSubscription {
    fn move_subscription(
        &self,
        subscription: &Subscription,
        old_folder: &OldFolder,
        new_folder: &NewFolder,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

impl MoveSubscription for Subscriptions {
    fn move_subscription(
        &self,
        subscription: &Subscription,
        old_folder: &OldFolder,
        new_folder: &NewFolder,
    ) -> Result<Self, Error> {
        let mut subs = self.clone();
        let duplicate_err = Error::set_error(
            "Cannot move subscription to",
            subscription.text.as_str(),
            "It already exists.",
        );
        match (old_folder, new_folder) {
            (None, Some(folder_new)) => {
                subs = Self::delete_subscription(
                    &subs,
                    &None,
                    &SubscriptionTitle(subscription.text.to_string()),
                );
                match Self::add_subscription(
                    &subs,
                    &Some(FolderName(folder_new.to_string())),
                    &SubscriptionTitle(subscription.text.to_string()),
                    &SubscriptionLink(subscription.xml_url.clone().unwrap()),
                ) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(duplicate_err),
                }
            }
            (Some(folder_old), None) => {
                subs = Self::delete_subscription(
                    &subs,
                    &Some(FolderName(folder_old.to_string())),
                    &SubscriptionTitle(subscription.text.to_string()),
                );
                match Self::add_subscription(
                    &subs,
                    &None,
                    &SubscriptionTitle(subscription.text.to_string()),
                    &SubscriptionLink(subscription.xml_url.clone().unwrap()),
                ) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(duplicate_err),
                }
            }
            (Some(folder_old), Some(folder_new)) => {
                subs = Self::delete_subscription(
                    &subs,
                    &Some(FolderName(folder_old.to_string())),
                    &SubscriptionTitle(subscription.text.to_string()),
                );
                match Self::add_subscription(
                    &subs,
                    &Some(FolderName(folder_new.to_string())),
                    &SubscriptionTitle(subscription.text.to_string()),
                    &SubscriptionLink(subscription.xml_url.clone().unwrap()),
                ) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(duplicate_err),
                }
            }
            (None, None) => Err(duplicate_err),
        }
    }
}

trait AddFeed {
    fn add_feed(&self, body: Vec<u8>) -> Result<Self, Error>
    where
        Self: Sized;
}

impl AddFeed for Subscriptions {
    fn add_feed(&self, body: Vec<u8>) -> Result<Self, Error> {
        let mut subs = self.clone();
        let feeds = subs.feeds.add_feed(body)?;
        subs.feeds = feeds;
        Ok(subs)
    }
}

trait FindFeed {
    fn find_feed(&self, feed_title: &SubscriptionTitle) -> Result<Feed, Error>;
}

impl FindFeed for Subscriptions {
    fn find_feed(&self, feed_title: &SubscriptionTitle) -> Result<Feed, Error> {
        self.feeds.find_feed(feed_title)
    }
}

impl Subscriptions {
    pub fn import(&self, opml_file_content: &str) -> Result<Self, Error> {
        Self::import_subscriptions(self, opml_file_content)
    }

    pub fn export(&self, subs_opml_name: &OpmlName) -> Result<String, Error> {
        Self::export_subscriptions(self, subs_opml_name)
    }

    pub fn add_folder(&self, folder_name: &FolderName) -> Result<Self, Error> {
        <Self as AddFolder>::add_folder(self, folder_name)
    }

    pub fn delete_folder(&self, folder_name: &FolderName) -> Self {
        <Self as DeleteFolder>::delete_folder(self, folder_name)
    }

    pub fn rename_folder(
        &self,
        old_folder_name: &OldFolderName,
        new_folder_name: &NewFolderName,
    ) -> Result<Self, Error> {
        <Self as RenameFolder>::rename_folder(self, old_folder_name, new_folder_name)
    }

    pub fn add_subscription(
        &self,
        folder_name: &Option<FolderName>,
        sub_title: &SubscriptionTitle,
        sub_link: &SubscriptionLink,
    ) -> Result<Self, Error> {
        <Self as AddSubscription>::add_subscription(self, folder_name, sub_title, sub_link)
    }

    pub fn delete_subscription(
        &self,
        folder_name: &Option<FolderName>,
        sub_title: &SubscriptionTitle,
    ) -> Self {
        <Self as DeleteSubscription>::delete_subscription(self, folder_name, sub_title)
    }

    pub fn rename_subscription(
        &self,
        folder_name: &Option<FolderName>,
        sub_link: &SubscriptionLink,
        old_sub_name: &OldSubscriptionName,
        new_sub_name: &NewSubscriptionName,
    ) -> Result<Self, Error> {
        <Self as RenameSubscription>::rename_subscription(
            self,
            folder_name,
            sub_link,
            old_sub_name,
            new_sub_name,
        )
    }

    pub fn move_subscription(
        &self,
        subscription: &Subscription,
        old_folder: &OldFolder,
        new_folder: &NewFolder,
    ) -> Result<Self, Error> {
        <Self as MoveSubscription>::move_subscription(self, subscription, old_folder, new_folder)
    }

    pub fn add_feed(&self, body: Vec<u8>) -> Result<Self, Error> {
        <Self as AddFeed>::add_feed(self, body)
    }

    pub fn find_feed(&self, feed_title: &SubscriptionTitle) -> Result<Feed, Error> {
        <Self as FindFeed>::find_feed(self, feed_title)
    }
}

#[cfg(test)]
mod import_export {
    use super::*;
    use crate::{Account, AccountType, Accounts};
    use crate::{App, Event, Model};
    use chrono::prelude::Local;
    use crux_core::App as _;
    use opml::OPML;

    #[test]
    fn import_subscriptions() {
        let app = App;
        let mut model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let example_import_opml = r#"<?xml version="1.0" encoding="ISO-8859-1"?> <opml version="2.0"> <head> <title>Subscriptions.opml</title> <dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated> <ownerName>Crab News</ownerName> </head> <body> <outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/> <outline text="Group Name" title="Group Name"> <outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/> </outline> </body> </opml>"#.to_string();
        let example_subs = r#"<opml version="2.0"><head><title>Subscriptions.opml</title><dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated><ownerName>Crab News</ownerName></head><body><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/><outline text="Group Name" title="Group Name"><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/></outline></body></opml>"#;

        let _ = app.update(
            Event::ImportSubscriptions(account.clone(), example_import_opml),
            &mut model,
            &(),
        );

        let added_subs = model.accounts.acct[account_index].subs.clone();
        let subs_feeds = added_subs.feeds.clone();

        let expected_subs = Subscriptions {
            subs: OPML::from_str(example_subs).unwrap(),
            feeds: subs_feeds,
        };

        assert_eq!(added_subs, expected_subs);
    }

    #[test]
    fn fail_import_for_invalid_xml() {
        let app = App;
        let mut model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let invalid_xml_opml = r#"<?xml version="2.0" encoding="ISO-8859-1"?> <opml version="2.0"> <head> <title>Subscriptions.opml</title> <dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated> <ownerName>Crab News</ownerName> </head> <body> <outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/> <outline text="Group Name" title="Group Name"> <outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/> </outline> </body> </opml>"#.to_string();

        let _ = app.update(
            Event::ImportSubscriptions(account, invalid_xml_opml),
            &mut model,
            &(),
        );
        let actual_error = model.notification.message;
        let expected_error = "Failed to process XML file";

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn fail_import_for_invalid_opml_version() {
        let app = App;
        let mut model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let invalid_version_opml =r#"<?xml version="1.0" encoding="ISO-8859-1"?> <opml version="0.1"> <head> <title>Subscriptions.opml</title> <dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated> <ownerName>Crab News</ownerName> </head> <body> <outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/> <outline text="Group Name" title="Group Name"> <outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/> </outline> </body> </opml>"#.to_string();

        let _ = app.update(
            Event::ImportSubscriptions(account, invalid_version_opml),
            &mut model,
            &(),
        );
        let actual_error = model.notification.message;
        let expected_error = "Unsupported OPML version: \"0.1\"";

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn fail_import_for_body_has_no_outlines() {
        let app = App;
        let mut model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let invalid_body_opml = r#"<?xml version="1.0" encoding="ISO-8859-1"?> <opml version="2.0"> <head> <title>Subscriptions.opml</title> <dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated> <ownerName>Crab News</ownerName> </head> <body> </body> </opml>"#.to_string();

        let _ = app.update(
            Event::ImportSubscriptions(account, invalid_body_opml),
            &mut model,
            &(),
        );
        let actual_error = model.notification.message;
        let expected_error = "OPML body has no <outline> elements";

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn export_subscriptions() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let date_created = Some(Local::now().format("%Y - %a %b %e %T").to_string());
        let subs_opml_name = OpmlName("Subscriptions.opml".to_string());

        #[allow(clippy::unnecessary_literal_unwrap)]
        let example_subs = format!("<opml version=\"2.0\"><head><title>{}</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName><ownerId>https://github.com/crab-apps/crab-news</ownerId></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", subs_opml_name, date_created.unwrap());

        model.accounts.acct[account_index].subs = Subscriptions {
            subs: OPML::from_str(&example_subs).unwrap(),
            feeds: Feeds::default(),
        };
        let imported_content = model.accounts.acct[account_index].subs.clone();

        let _ = app.update(
            Event::ExportSubscriptions(account, subs_opml_name.clone()),
            &mut model,
            &(),
        );

        // TODO use proper Shell/WASM/crate functionality to File operations
        let mut exported_file = std::fs::File::open(subs_opml_name.0).unwrap();
        let exported_content = Subscriptions {
            subs: OPML::from_reader(&mut exported_file).unwrap(),
            feeds: Feeds::default(),
        };

        assert_eq!(exported_content, imported_content);
    }

    #[test]
    fn export_subscriptions_notification() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let date_created = Some(Local::now().format("%Y - %a %b %e %T").to_string());
        let subs_opml_name = OpmlName("Subscriptions.opml".to_string());

        #[allow(clippy::unnecessary_literal_unwrap)]
        let example_subs = format!("<opml version=\"2.0\"><head><title>{:?}</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName><ownerId>https://github.com/crab-apps/crab-news</ownerId></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", subs_opml_name, date_created.unwrap());

        model.accounts.acct[account_index].subs = Subscriptions {
            subs: OPML::from_str(&example_subs).unwrap(),
            feeds: Feeds::default(),
        };

        let _ = app.update(
            Event::ExportSubscriptions(account, subs_opml_name),
            &mut model,
            &(),
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
    //         &mut model, &()
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
    use crate::{Account, AccountType, Accounts};
    use crate::{App, Event, Model};
    use crux_core::App as _;
    use opml::Outline;

    #[test]
    fn add_new_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let folder_name = FolderName("Added Folder".to_string());
        let added_folder = &Outline {
            text: folder_name.to_string(),
            title: Some(folder_name.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(account, folder_name), &mut model, &());

        let does_contain_new_folder = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .contains(added_folder);

        assert!(does_contain_new_folder);
    }

    #[test]
    fn add_two_new_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let folder_name_one = FolderName("Added Folder One".to_string());
        let folder_name_two = FolderName("Added Folder Two".to_string());
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
            Event::AddNewFolder(account.clone(), folder_name_one),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddNewFolder(account, folder_name_two),
            &mut model,
            &(),
        );

        let does_contain_folder_one = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .contains(added_folder_one);
        let does_contain_folder_two = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .contains(added_folder_two);

        assert!((does_contain_folder_one && does_contain_folder_two));
    }

    #[test]
    fn fail_add_new_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let folder_name = FolderName("Added Folder".to_string());

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let actual_error = model.notification.message;
        let expected_error = format!("Cannot add new folder \"{folder_name}\". It already exists.");

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn delete_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let deleted_folder = &Outline {
            text: "Deleted Folder".to_string(),
            title: Some("Deleted Folder".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), FolderName(deleted_folder.text.to_string())),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::DeleteFolder(account.clone(), FolderName(deleted_folder.text.to_string())),
            &mut model,
            &(),
        );

        let does_not_contain_deleted_folder = !model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .contains(deleted_folder);

        assert!(does_not_contain_deleted_folder);
    }

    #[test]
    fn rename_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
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
            Event::AddNewFolder(account.clone(), FolderName(rename_folder.text.to_string())),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::RenameFolder(
                account.clone(),
                OldFolderName(rename_folder.text.to_string()),
                NewFolderName(expected_folder.text.to_string()),
            ),
            &mut model,
            &(),
        );

        let does_contain_renamed_folder = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .contains(expected_folder);

        assert!(does_contain_renamed_folder);
    }

    #[test]
    fn fail_rename_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let test_folder = &Outline {
            text: "Expected Folder".to_string(),
            title: Some("Expected Folder".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), FolderName(test_folder.text.to_string())),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::RenameFolder(
                account.clone(),
                OldFolderName(test_folder.text.to_string()),
                NewFolderName(test_folder.text.to_string()),
            ),
            &mut model,
            &(),
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
    use crate::{Account, AccountType, Accounts};
    use crate::{App, Event, Model};
    use crux_core::App as _;
    use opml::Outline;

    #[test]
    fn add_new_subscription_to_root() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let sub_title = SubscriptionTitle("New Sub Root".to_string());
        let sub_link = SubscriptionLink("https://example.com/atom.xml".to_string());
        let expected_sub = &Outline {
            text: sub_title.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddSubscription(account.clone(), None, sub_title, sub_link),
            &mut model,
            &(),
        );

        let does_root_contain_new_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .contains(expected_sub);

        assert!(does_root_contain_new_sub);
    }

    #[test]
    fn add_new_subscription_to_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let folder_name = FolderName("New Sub Folder".to_string());
        let sub_title = SubscriptionTitle("New Sub Folder".to_string());
        let sub_link = SubscriptionLink("https://example.com/atom.xml".to_string());
        let expected_sub = &Outline {
            text: sub_title.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                sub_title,
                sub_link,
            ),
            &mut model,
            &(),
        );

        let does_folder_contain_new_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .any(|outline| {
                outline.text == folder_name.to_string() && outline.outlines.contains(expected_sub)
            });

        assert!(does_folder_contain_new_sub);
    }

    #[test]
    fn fail_add_new_subscription_to_root() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let sub_title = SubscriptionTitle("New Sub Root".to_string());
        let sub_link = SubscriptionLink("https://example.com/atom.xml".to_string());
        let test_subscription = &Outline {
            text: sub_title.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddSubscription(account.clone(), None, sub_title.clone(), sub_link.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(account.clone(), None, sub_title, sub_link),
            &mut model,
            &(),
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
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let folder_name = FolderName("New Sub Folder".to_string());
        let sub_title = SubscriptionTitle("New Sub Folder".to_string());
        let sub_link = SubscriptionLink("https://example.com/atom.xml".to_string());
        let test_subscription = &Outline {
            text: sub_title.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                sub_title.clone(),
                sub_link.clone(),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(account.clone(), Some(folder_name), sub_title, sub_link),
            &mut model,
            &(),
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
    use crate::{Account, AccountType, Accounts};
    use crate::{App, Event, Model};
    use crux_core::App as _;
    use opml::Outline;

    #[test]
    fn delete_subscription_from_root() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let deleted_sub = &Outline {
            text: "Deleted Sub Root".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                SubscriptionTitle(deleted_sub.text.to_string()),
                SubscriptionLink(deleted_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::DeleteSubscription(
                account.clone(),
                None,
                SubscriptionTitle(deleted_sub.text.to_string()),
            ),
            &mut model,
            &(),
        );

        let does_root_not_contain_sub = !model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .contains(deleted_sub);

        assert!(does_root_not_contain_sub);
    }

    #[test]
    fn delete_subscription_from_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let folder_name = FolderName("Deleted Sub Folder".to_string());
        let deleted_sub = &Outline {
            text: "Sub Name".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(deleted_sub.text.to_string()),
                SubscriptionLink(deleted_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::DeleteSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(deleted_sub.text.to_string()),
            ),
            &mut model,
            &(),
        );

        let does_folder_not_contain_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .any(|outline| {
                outline.text == folder_name.to_string() && !outline.outlines.contains(deleted_sub)
            });

        assert!(does_folder_not_contain_sub);
    }

    #[test]
    fn delete_subscription_from_folder_with_multi_subs() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let folder_name = FolderName("Deleted Multi Subs".to_string());
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
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(delete_sub.text.to_string()),
                SubscriptionLink(delete_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::DeleteSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(delete_sub.text.to_string()),
            ),
            &mut model,
            &(),
        );

        let does_folder_not_contain_deleted_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .any(|outline| {
                outline.text == folder_name.to_string() && !outline.outlines.contains(delete_sub)
            });

        let does_folder_contain_expected_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .any(|outline| {
                outline.text == folder_name.to_string() && outline.outlines.contains(expected_sub)
            });

        assert!((does_folder_not_contain_deleted_sub && does_folder_contain_expected_sub));
    }
}

#[cfg(test)]
mod rename_subscription {
    use super::*;
    use crate::{Account, AccountType, Accounts};
    use crate::{App, Event, Model};
    use crux_core::App as _;
    use opml::Outline;

    #[test]
    fn rename_subscription_in_root() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
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
                SubscriptionTitle(rename_sub.text.to_string()),
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                None,
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
                OldSubscriptionName(rename_sub.text.to_string()),
                NewSubscriptionName(expected_sub.text.to_string()),
            ),
            &mut model,
            &(),
        );

        let does_root_contain_renamed_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .contains(expected_sub);

        assert!(does_root_contain_renamed_sub);
    }

    #[test]
    fn fail_rename_subscription_in_root() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                SubscriptionTitle(rename_sub.text.to_string()),
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                None,
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
                OldSubscriptionName(rename_sub.text.to_string()),
                NewSubscriptionName(rename_sub.text.to_string()),
            ),
            &mut model,
            &(),
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
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let folder_name = FolderName("Renamed Sub Folder".to_string());
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
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(rename_sub.text.to_string()),
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
                OldSubscriptionName(rename_sub.text.to_string()),
                NewSubscriptionName(expected_sub.text.to_string()),
            ),
            &mut model,
            &(),
        );

        let does_folder_contain_renamed_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .any(|outline| {
                outline.text == folder_name.to_string() && outline.outlines.contains(expected_sub)
            });

        assert!(does_folder_contain_renamed_sub);
    }

    #[test]
    fn fail_rename_subscription_in_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let folder_name = FolderName("Renamed Sub Folder".to_string());
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(rename_sub.text.to_string()),
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
                OldSubscriptionName(rename_sub.text.to_string()),
                NewSubscriptionName(rename_sub.text.to_string()),
            ),
            &mut model,
            &(),
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
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let folder_name = FolderName("Renamed Multi Sub Folder".to_string());
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
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(untouched_sub.text.to_string()),
                SubscriptionLink(untouched_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
                OldSubscriptionName(rename_sub.text.to_string()),
                NewSubscriptionName(expected_sub.text.to_string()),
            ),
            &mut model,
            &(),
        );

        let does_folder_contain_untouched_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .any(|outline| {
                outline.text == folder_name.to_string() && outline.outlines.contains(untouched_sub)
            });

        let does_folder_contain_renamed_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .any(|outline| {
                outline.text == folder_name.to_string() && outline.outlines.contains(expected_sub)
            });

        assert!((does_folder_contain_untouched_sub && does_folder_contain_renamed_sub));
    }

    #[test]
    fn fail_rename_subscription_in_folder_with_multi_subs() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let folder_name = FolderName("Renamed Multi Sub Folder".to_string());
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
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(untouched_sub.text.to_string()),
                SubscriptionLink(untouched_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(rename_sub.text.to_string()),
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::RenameSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionLink(rename_sub.xml_url.clone().unwrap().clone()),
                OldSubscriptionName(rename_sub.text.to_string()),
                NewSubscriptionName(rename_sub.text.to_string()),
            ),
            &mut model,
            &(),
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
    use crate::{Account, AccountType, Accounts};
    use crate::{App, Event, Model};
    use crux_core::App as _;
    use opml::Outline;

    #[test]
    fn move_subscription_from_root_to_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let folder_name = FolderName("Move Sub To Folder".to_string());
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                None,
                Some(folder_name.clone()),
            ),
            &mut model,
            &(),
        );

        let does_root_not_contain_sub = !model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .contains(expected_sub);

        let does_folder_contain_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .any(|outline| {
                outline.text == folder_name.to_string() && outline.outlines.contains(expected_sub)
            });

        assert!(does_root_not_contain_sub && does_folder_contain_sub);
    }

    #[test]
    fn fail_move_subscription_from_root_to_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let folder_name = FolderName("Move Sub To Folder".to_string());
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                None,
                Some(folder_name.clone()),
            ),
            &mut model,
            &(),
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
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let folder_name = FolderName("Move Sub To Root".to_string());
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                Some(folder_name.clone()),
                None,
            ),
            &mut model,
            &(),
        );

        let does_root_contain_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .contains(expected_sub);

        let does_folder_not_contain_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .all(|outline| {
                outline.text == folder_name.to_string() || !outline.outlines.contains(expected_sub)
            });

        assert!(does_root_contain_sub && does_folder_not_contain_sub);
    }

    #[test]
    fn fail_move_subscription_from_folder_to_root() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let folder_name = FolderName("Move Sub To Root".to_string());
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_name.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_name.clone()),
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                None,
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                Some(folder_name.clone()),
                None,
            ),
            &mut model,
            &(),
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
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let account_index = Accounts::find_by_index(&model.accounts, &account);
        let folder_one = FolderName("Folder One".to_string());
        let folder_two = FolderName("Folder Two".to_string());
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_one.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_two.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_one.clone()),
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                Some(folder_one.clone()),
                Some(folder_two.clone()),
            ),
            &mut model,
            &(),
        );

        let does_folder_one_not_contain_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .any(|outline| {
                outline.text == folder_one.to_string() && !outline.outlines.contains(expected_sub)
            });

        let does_folder_two_contain_sub = model.accounts.acct[account_index]
            .subs
            .subs
            .body
            .outlines
            .iter()
            .any(|outline| {
                outline.text == folder_two.to_string() && outline.outlines.contains(expected_sub)
            });

        assert!(does_folder_one_not_contain_sub && does_folder_two_contain_sub);
    }

    #[test]
    fn fail_move_subscription_from_folder_to_folder() {
        let app = App;
        let mut model: Model = Model::default();
        let account = Account::new(&AccountType::Local);

        let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
        let folder_one = FolderName("Folder One".to_string());
        let folder_two = FolderName("Folder Two".to_string());
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/atom.xml".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_one.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddNewFolder(account.clone(), folder_two.clone()),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_one.clone()),
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::AddSubscription(
                account.clone(),
                Some(folder_two.clone()),
                SubscriptionTitle(expected_sub.text.to_string()),
                SubscriptionLink(expected_sub.xml_url.clone().unwrap().clone()),
            ),
            &mut model,
            &(),
        );
        let _ = app.update(
            Event::MoveSubscription(
                account.clone(),
                expected_sub.clone(),
                Some(folder_one.clone()),
                Some(folder_two.clone()),
            ),
            &mut model,
            &(),
        );

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot move subscription to \"{}\". It already exists.",
            expected_sub.text
        );

        assert_eq!(actual_error, expected_error);
    }
}
