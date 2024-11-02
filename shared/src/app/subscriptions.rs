use chrono::Local;
use opml::{Head, Outline, OPML};
use serde::{Deserialize, Serialize};
use std::fs::File;

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

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Subscriptions {
    pub opml: OPML,
}

impl Subscriptions {
    pub fn is_duplicate(
        &mut self,
        folder_name: Option<FolderName>,
        sub_name: Option<SubscriptionName>,
        sub_url: Option<SubscriptionURL>,
    ) -> bool {
        let mut is_duplicate = false;

        match (folder_name, sub_name, sub_url) {
            (Some(folder_text), None, None) => {
                let folder = &Outline {
                    text: folder_text.clone(),
                    title: Some(folder_text.clone()),
                    ..Outline::default()
                };
                is_duplicate = self.opml.body.outlines.contains(folder);
            }
            (Some(folder_text), Some(sub_text), Some(sub_link)) => {
                let subscription = &Outline {
                    text: sub_text.to_string(),
                    xml_url: Some(sub_link.to_string()),
                    ..Outline::default()
                };
                let _ = self
                    .opml
                    .body
                    .outlines
                    .iter_mut()
                    .filter(|outline| outline.text == folder_text)
                    .for_each(|folder| {
                        is_duplicate = folder.outlines.contains(subscription);
                    });
            }
            (None, Some(sub_text), Some(sub_link)) => {
                let subscription = &Outline {
                    text: sub_text.to_string(),
                    xml_url: Some(sub_link.to_string()),
                    ..Outline::default()
                };
                is_duplicate = self.opml.body.outlines.contains(subscription);
            }
            _ => {
                let _ = self.opml;
            }
        }
        is_duplicate
    }

    pub fn import(&mut self, subs_opml_file: OpmlFile) {
        let mut file = File::open(subs_opml_file).unwrap();
        self.opml = OPML::from_reader(&mut file).unwrap();
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

    pub fn add_folder(&mut self, folder_name: FolderName) {
        let new_folder = Outline {
            text: folder_name.clone(),
            title: Some(folder_name.clone()),
            ..Outline::default()
        };
        self.opml.body.outlines.push(new_folder);
    }

    pub fn delete_folder(&mut self, folder_name: FolderName) {
        self.opml
            .body
            .outlines
            .retain(|name| name.text != folder_name);
    }

    pub fn rename_folder(&mut self, old_folder_name: OldName, new_folder_name: NewName) {
        self.opml
            .body
            .outlines
            .iter_mut()
            .filter(|outline| outline.text == old_folder_name)
            .for_each(|folder| {
                folder.text = new_folder_name.clone();
                folder.title = Some(new_folder_name.clone());
            });
    }

    pub fn add_subscription(
        &mut self,
        folder_name: Option<FolderName>,
        sub_name: SubscriptionName,
        sub_url: SubscriptionURL,
    ) {
        if let Some(folder_text) = folder_name {
            self.opml
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == folder_text)
                .for_each(|folder| {
                    folder.add_feed(sub_name.as_str(), sub_url.as_str());
                });
        } else {
            self.opml.add_feed(sub_name.as_str(), sub_url.as_str());
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
    ) {
        if let Some(folder_text) = folder_name {
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
        } else {
            self.opml
                .body
                .outlines
                .iter_mut()
                .filter(|outline| outline.text == old_name)
                .for_each(|sub| sub.text = new_name.clone());
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
                Self::add_subscription(
                    self,
                    Some(folder_new),
                    subscription.text.clone(),
                    subscription.xml_url.unwrap(),
                );
                Self::delete_subscription(self, None, subscription.text.clone());
            }
            (Some(folder_old), None) => {
                Self::add_subscription(
                    self,
                    None,
                    subscription.text.clone(),
                    subscription.xml_url.unwrap(),
                );
                Self::delete_subscription(self, Some(folder_old), subscription.text.clone());
            }
            (Some(folder_old), Some(folder_new)) => {
                Self::add_subscription(
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
