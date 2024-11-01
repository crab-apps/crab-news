use chrono::Local;
use opml::{Head, Outline, OPML};
use serde::{Deserialize, Serialize};
use std::{fs::File, slice::IterMut};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Subscriptions {
    pub opml: OPML,
}

impl Subscriptions {
    pub fn new() -> Self {
        Subscriptions {
            opml: OPML::default(),
        }
    }

    // ANCHOR: helper functions
    fn body_outlines_iter_mut(&mut self) -> IterMut<'_, Outline> {
        self.opml.body.outlines.iter_mut()
    }

    // fn find_duplicates() -> i32 {}
    // ANCHOR_END: helper functions

    pub fn import(&mut self, subs_opml_file: String) {
        let mut file = File::open(subs_opml_file).unwrap();
        self.opml = OPML::from_reader(&mut file).unwrap();
    }

    pub fn export(&self, subs_opml_name: String) {
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

    pub fn add_folder(&mut self, folder_name: String) {
        let new_folder = Outline {
            text: folder_name.clone(),
            title: Some(folder_name.clone()),
            ..Outline::default()
        };
        self.opml.body.outlines.push(new_folder);
    }

    pub fn delete_folder(&mut self, folder_name: String) {
        self.opml
            .body
            .outlines
            .retain(|name| name.text != folder_name);
    }

    pub fn rename_folder(&mut self, old_folder_name: String, new_folder_name: String) {
        Self::body_outlines_iter_mut(self)
            .filter(|outline| outline.text == old_folder_name)
            .for_each(|folder| {
                folder.text = new_folder_name.clone();
                folder.title = Some(new_folder_name.clone());
            });
    }

    pub fn add_subscription(
        &mut self,
        folder_name: Option<String>,
        sub_name: String,
        sub_url: String,
    ) {
        if let Some(folder_text) = folder_name {
            Self::body_outlines_iter_mut(self)
                .filter(|outline| outline.text == folder_text)
                .for_each(|folder| {
                    folder.add_feed(sub_name.as_str(), sub_url.as_str());
                });
        } else {
            self.opml.add_feed(sub_name.as_str(), sub_url.as_str());
        }
    }

    pub fn delete_subscription(&mut self, folder_name: Option<String>, sub_name: String) {
        if let Some(folder_text) = folder_name {
            Self::body_outlines_iter_mut(self)
                .filter(|outline| outline.text == folder_text)
                .for_each(|folder| folder.outlines.retain(|name| name.text != sub_name));
        } else {
            self.opml.body.outlines.retain(|name| name.text != sub_name);
        }
    }

    pub fn rename_subscription(
        &mut self,
        folder_name: Option<String>,
        old_name: String,
        new_name: String,
    ) {
        if let Some(folder_text) = folder_name {
            Self::body_outlines_iter_mut(self)
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
            Self::body_outlines_iter_mut(self)
                .filter(|outline| outline.text == old_name)
                .for_each(|sub| sub.text = new_name.clone());
        }
    }

    pub fn move_subscription(
        &mut self,
        subscription: Outline,
        old_folder: Option<String>,
        new_folder: Option<String>,
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
