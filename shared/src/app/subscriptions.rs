use chrono::Local;
use opml::{self, Head, Outline, OPML};
use serde::{Deserialize, Serialize};
use std::fs::File;
use thiserror::Error; // because https://github.com/Holllo/opml/blob/main/opml_api/Cargo.toml

// ANCHOR: type aliases
pub type OpmlFile = String;
pub type OpmlName = String;
pub type FolderName = String;
pub type OldName = String;
pub type NewName = String;
pub type OldFolder = Option<FolderName>;
pub type NewFolder = Option<FolderName>;
pub type Subscription = Outline;
pub type SubscriptionName = String;
pub type SubscriptionURL = String;
// ANCHOR_END: types aliases

// https://github.com/Holllo/opml/issues/5
#[derive(Debug, Error)]
pub enum CustomErrors {
    #[error("Cannot {action} {outline}. {subject} already exists.")]
    OutlineAlreadyExists {
        action: String,
        outline: String,
        subject: String,
    },
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Subscriptions {
    pub opml: OPML,
}

impl Subscriptions {
    pub fn import(&mut self, subs_opml_file: OpmlFile) -> Result<&mut Self, opml::Error> {
        let mut file = File::open(subs_opml_file).unwrap();
        self.opml = match OPML::from_reader(&mut file) {
            Ok(subscriptions) => subscriptions,
            Err(error) => return Err(error),
        };
        Ok(self)
    }

    pub fn export(&self, subs_opml_name: OpmlName) {
        let xml_tag = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string();
        let custom_head = Head {
            title: Some(subs_opml_name.clone()),
            date_created: Some(Local::now().format("%Y - %a %b %e %T").to_string()),
            owner_name: Some("Crab News".to_string()),
            ..Head::default()
        };
        let custon_opml = OPML {
            head: Some(custom_head),
            body: self.opml.body.clone(),
            ..OPML::default()
        };
        let export_content = xml_tag + &custon_opml.to_string().unwrap();
        let _ = std::fs::write(subs_opml_name, &export_content);
    }

    pub fn add_folder(&mut self, folder_name: FolderName) -> Result<&mut Self, self::CustomErrors> {
        let test_folder = Outline {
            text: folder_name.clone(),
            title: Some(folder_name.clone()),
            ..Outline::default()
        };

        if self.opml.body.outlines.contains(&test_folder) {
            return Err(self::CustomErrors::OutlineAlreadyExists {
                action: "add new folder".to_string(),
                outline: folder_name.to_string(),
                subject: "It".to_string(),
            });
        } else {
            self.opml.body.outlines.push(test_folder);
            Ok(self)
        }
    }

    pub fn delete_folder(&mut self, folder_name: FolderName) {
        self.opml
            .body
            .outlines
            .retain(|name| name.text != folder_name);
    }

    pub fn rename_folder(
        &mut self,
        old_folder_name: OldName,
        new_folder_name: NewName,
    ) -> Result<&mut Self, self::CustomErrors> {
        let test_folder = Outline {
            text: new_folder_name.clone(),
            title: Some(new_folder_name.clone()),
            ..Outline::default()
        };

        if self.opml.body.outlines.contains(&test_folder) {
            return Err(self::CustomErrors::OutlineAlreadyExists {
                action: format!("rename folder {} to", old_folder_name.to_string()),
                outline: new_folder_name.to_string(),
                subject: format!("{}", new_folder_name.to_string()),
            });
        } else {
            self.opml
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == old_folder_name)
                .for_each(|folder| {
                    folder.text = new_folder_name.clone();
                    folder.title = Some(new_folder_name.clone());
                });
            Ok(self)
        }
    }

    pub fn add_subscription(
        &mut self,
        folder_name: Option<FolderName>,
        sub_name: SubscriptionName,
        sub_url: SubscriptionURL,
    ) -> Result<&mut Self, self::CustomErrors> {
        let test_subscription = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_url.to_string()),
            ..Outline::default()
        };

        if let Some(folder_text) = folder_name {
            if self
                .opml
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == folder_text)
                .find_map(|folder| Some(folder.outlines.contains(test_subscription)))
                .unwrap()
            {
                return Err(self::CustomErrors::OutlineAlreadyExists {
                    action: "add new subscription".to_string(),
                    outline: sub_name.to_string(),
                    subject: "It".to_string(),
                });
            } else {
                self.opml
                    .body
                    .outlines
                    .iter_mut()
                    .filter(|outline| outline.text == folder_text)
                    .for_each(|folder| {
                        folder.add_feed(sub_name.as_str(), sub_url.as_str());
                    });
                Ok(self)
            }
        } else {
            if self.opml.body.outlines.contains(test_subscription) {
                return Err(self::CustomErrors::OutlineAlreadyExists {
                    action: "add new subscription".to_string(),
                    outline: sub_name.to_string(),
                    subject: "It".to_string(),
                    // subject: format!("{}", new_folder_name.to_string()),
                });
            } else {
                self.opml.add_feed(sub_name.as_str(), sub_url.as_str());
                Ok(self)
            }
        }
    }

    pub fn delete_subscription(
        &mut self,
        folder_name: Option<FolderName>,
        sub_name: SubscriptionName,
    ) {
        if let Some(folder_text) = folder_name {
            self.opml
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == folder_text)
                .for_each(|folder| folder.outlines.retain(|name| name.text != sub_name));
        } else {
            self.opml.body.outlines.retain(|name| name.text != sub_name);
        }
    }

    pub fn rename_subscription(
        &mut self,
        folder_name: Option<FolderName>,
        old_name: OldName,
        new_name: NewName,
    ) -> Result<&mut Self, self::CustomErrors> {
        let test_subscription = &Outline {
            text: new_name.to_string(),
            ..Outline::default()
        };
        if let Some(folder_text) = folder_name {
            if self
                .opml
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == folder_text)
                .find_map(|folder| Some(folder.outlines.contains(test_subscription)))
                .unwrap()
            {
                return Err(self::CustomErrors::OutlineAlreadyExists {
                    action: format!("rename subscription {} to", old_name.to_string()),
                    outline: new_name.to_string(),
                    subject: format!("{}", new_name.to_string()),
                });
            } else {
                self.opml
                    .body
                    .outlines
                    .iter_mut()
                    .filter(|outline| outline.text == folder_text)
                    .for_each(|folder| {
                        folder
                            .outlines
                            .iter_mut()
                            .filter(|sub| sub.text == old_name)
                            .for_each(|sub| {
                                sub.text = new_name.clone();
                            });
                    });
                Ok(self)
            }
        } else {
            if self.opml.body.outlines.contains(test_subscription) {
                return Err(self::CustomErrors::OutlineAlreadyExists {
                    action: format!("rename subscription {} to", old_name.to_string()),
                    outline: new_name.to_string(),
                    subject: format!("{}", new_name.to_string()),
                });
            } else {
                self.opml
                    .body
                    .outlines
                    .iter_mut()
                    .filter(|outline| outline.text == old_name)
                    .for_each(|sub| sub.text = new_name.clone());
                Ok(self)
            }
        }
    }

    pub fn move_subscription(
        &mut self,
        subscription: Subscription,
        old_folder: OldFolder,
        new_folder: NewFolder,
    ) {
        match (old_folder, new_folder) {
            (None, Some(folder_new)) => {
                let _ = Self::add_subscription(
                    self,
                    Some(folder_new),
                    subscription.text.clone(),
                    subscription.xml_url.unwrap(),
                );
                Self::delete_subscription(self, None, subscription.text.clone());
            }
            (Some(folder_old), None) => {
                let _ = Self::add_subscription(
                    self,
                    None,
                    subscription.text.clone(),
                    subscription.xml_url.unwrap(),
                );
                Self::delete_subscription(self, Some(folder_old), subscription.text.clone());
            }
            (Some(folder_old), Some(folder_new)) => {
                let _ = Self::add_subscription(
                    self,
                    Some(folder_new),
                    subscription.text.clone(),
                    subscription.xml_url.unwrap(),
                );
                Self::delete_subscription(self, Some(folder_old), subscription.text.clone());
            }
            _ => panic!(),
        }
    }
}
