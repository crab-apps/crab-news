use chrono::Local;
use opml::{self, Head, Outline, OPML};
use serde::{Deserialize, Serialize};
use std::fs::{write, File};
use std::io;
use thiserror::Error; // because https://github.com/Holllo/opml/blob/main/opml_api/Cargo.toml

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
// ANCHOR_END: types aliases

// https://github.com/Holllo/opml/issues/5
#[derive(Debug, Error)]
pub enum Error {
    #[error("{action} \"{outline}\". {reason}")]
    OutlineAlreadyExists {
        action: String,
        outline: String,
        reason: String,
    },
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Subscriptions {
    pub opml: OPML,
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

    fn set_duplicate_err(action: &str, outline: &str, reason: &str) -> self::Error {
        self::Error::OutlineAlreadyExists {
            action: action.to_string(),
            outline: outline.to_string(),
            reason: reason.to_string(),
        }
    }
    // ANCHOR_END: helper functions

    // TODO on duplicates, prompt user for merge or replace
    pub fn import(&self, subs_opml_file: OpmlFile) -> Result<Self, opml::Error> {
        // TODO use proper Shell/WASM functionality to pass on File operations
        let mut file = File::open(subs_opml_file).unwrap();
        Ok(Self {
            opml: OPML::from_reader(&mut file)?,
        })
    }

    // TODO once shell is implemented, check failures
    pub fn export(&self, subs_opml_name: OpmlName) -> Result<String, io::Error> {
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
            body: self.opml.body.clone(),
            ..OPML::default()
        };
        let export_content = xml_tag + &custom_opml.to_string().unwrap();
        // TODO use proper Shell/WASM functionality to pass on File operations
        match write(subs_opml_name, &export_content) {
            Ok(_) => Ok("Subscriptions successfully exported".to_string()),
            Err(e) => return Err(e),
        }
    }

    // NOTE folders are only allowed at root level. no nesting.
    pub fn add_folder(&self, folder_name: FolderName) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let test_folder = Self::set_test_folder(&folder_name);
        let duplicate_err = Self::set_duplicate_err(
            "Cannot add new folder",
            folder_name.as_str(),
            "It already exists.",
        );

        if subs.opml.body.outlines.contains(&test_folder) {
            Err(duplicate_err)
        } else {
            subs.opml.body.outlines.push(test_folder);
            Ok(subs)
        }
    }

    // NOTE folders are only allowed at root level. no nesting.
    pub fn delete_folder(&self, folder_name: FolderName) -> Self {
        let mut subs = self.clone();
        subs.opml
            .body
            .outlines
            .retain(|name| name.text != folder_name);
        subs
    }

    // NOTE folders are only allowed at root level. no nesting.
    pub fn rename_folder(
        &self,
        old_folder_name: OldName,
        new_folder_name: NewName,
    ) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let test_folder = Self::set_test_folder(&new_folder_name);
        let duplicate_err = Self::set_duplicate_err(
            "Cannot rename folder to",
            new_folder_name.as_str(),
            "It already exists.",
        );

        if subs.opml.body.outlines.contains(&test_folder) {
            Err(duplicate_err)
        } else {
            subs.opml
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == old_folder_name)
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
        folder_name: Option<FolderName>,
        sub_title: SubscriptionTitle,
        sub_link: SubscriptionLink,
    ) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let test_subscription = Self::set_test_sub(&sub_title, &sub_link);
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
            subs.opml
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

        if subs.opml.body.outlines.contains(&test_subscription) {
            Err(duplicate_err)
        } else {
            subs.opml.add_feed(sub_title.as_str(), sub_link.as_str());
            Ok(subs)
        }
    }

    pub fn delete_subscription(
        &self,
        folder_name: Option<FolderName>,
        sub_title: SubscriptionTitle,
    ) -> Self {
        let mut subs = self.clone();
        if let Some(folder_text) = folder_name {
            subs.opml
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == folder_text)
                .for_each(|folder| folder.outlines.retain(|name| name.text != sub_title));
        } else {
            subs.opml
                .body
                .outlines
                .retain(|name| name.text != sub_title);
        }
        subs
    }

    // NOTE rename to an existing sub should always fail no matter where it exists
    pub fn rename_subscription(
        &self,
        folder_name: Option<FolderName>,
        old_name: OldName,
        old_link: OldLink,
        new_name: NewName,
    ) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let test_subscription = Self::set_test_sub(&new_name, &old_link);
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
            subs.opml
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
                            .filter(|sub| sub.text == old_name)
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

        if subs.opml.body.outlines.contains(&test_subscription) {
            Err(duplicate_err)
        } else {
            subs.opml
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == old_name)
                .for_each(|sub| sub.text = new_name.to_string());
            Ok(subs)
        }
    }

    pub fn move_subscription(
        &self,
        subscription: Subscription,
        old_folder: OldFolder,
        new_folder: NewFolder,
    ) -> Result<Self, self::Error> {
        let mut subs = self.clone();
        let duplicate_err = Self::set_duplicate_err(
            "Cannot move subscription to",
            subscription.text.as_str(),
            "It already exists.",
        );
        match (old_folder, new_folder) {
            (None, Some(folder_new)) => {
                subs = Self::delete_subscription(&subs, None, subscription.text.to_string());
                match Self::add_subscription(
                    &subs,
                    Some(folder_new),
                    subscription.text.to_string(),
                    subscription.xml_url.unwrap(),
                ) {
                    Ok(s) => Ok(s),
                    Err(_) => return Err(duplicate_err),
                }
            }
            (Some(folder_old), None) => {
                subs = Self::delete_subscription(
                    &subs,
                    Some(folder_old),
                    subscription.text.to_string(),
                );
                match Self::add_subscription(
                    &subs,
                    None,
                    subscription.text.to_string(),
                    subscription.xml_url.unwrap(),
                ) {
                    Ok(s) => Ok(s),
                    Err(_) => return Err(duplicate_err),
                }
            }
            (Some(folder_old), Some(folder_new)) => {
                subs = Self::delete_subscription(
                    &subs,
                    Some(folder_old),
                    subscription.text.to_string(),
                );
                match Self::add_subscription(
                    &subs,
                    Some(folder_new),
                    subscription.text.to_string(),
                    subscription.xml_url.unwrap(),
                ) {
                    Ok(s) => Ok(s),
                    Err(_) => return Err(duplicate_err),
                }
            }
            (None, None) => {
                return Err(duplicate_err);
            }
        }
    }
}
