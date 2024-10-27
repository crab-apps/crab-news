// ANCHOR: app
use chrono::prelude::Local;
use crux_core::{render::Render, App};
use feed_rs::model::Feed;
use opml::{Head, Outline, OPML};
use serde::{Deserialize, Serialize};
use std::{fs::File, slice::IterMut};
// use crux_http::Http;
// use url::Url;

// ANCHOR: constants
const OPML_DATE_FORMAT: &str = "%Y - %a %b %e %T";
// ANCHOR_END: constants

// ANCHOR: events
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    ImportSubscriptions(OpmlFile),
    ExportSubscriptions(OpmlName),
    AddNewFolder(FolderName),
    DeleteFolder(FolderName),
    RenameFolder(OldName, NewName),
    AddNewSubscription(Option<FolderName>, SubscriptionName, SubscriptionURL),
    DeleteSubscription(Option<FolderName>, SubscriptionName),
    RenameSubscription(Option<FolderName>, OldName, NewName),
    MoveSubscriptionToFolder(Subscription, OldFolder, NewFolder),

    // EVENTS LOCAL TO THE CORE
    #[serde(skip)]
    Fetch(crux_http::Result<crux_http::Response<Feed>>),
}
// ANCHOR_END: events

// ANCHOR: types
type OpmlFile = String;
type OpmlName = String;
type FolderName = String;
type OldName = String;
type NewName = String;
type Subscription = Outline;
type SubscriptionName = String;
type SubscriptionURL = String;
type OldFolder = Option<FolderName>;
type NewFolder = Option<FolderName>;
// ANCHOR_END: types

// ANCHOR: model
#[derive(Default)]
pub struct Model {
    subscriptions: OPML,
}
// ANCHOR_END: model

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ViewModel {
    // pub subscription_folder: String,
    // pub subscription_name: String,
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Default)]
pub struct CrabNews;

// ANCHOR: traits
trait BodyOutlineIter {
    fn body_outlines_iter_mut(model: &mut Model) -> IterMut<'_, Outline>;
}

impl BodyOutlineIter for CrabNews {
    fn body_outlines_iter_mut(model: &mut Model) -> IterMut<'_, Outline> {
        model.subscriptions.body.outlines.iter_mut()
    }
}
// ANCHOR_END: traits

// ANCHOR: impl_app
impl App for CrabNews {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match event {
            Event::ImportSubscriptions(subs_opml_file) => {
                let mut file = File::open(subs_opml_file).unwrap();
                model.subscriptions = OPML::from_reader(&mut file).unwrap();
            }
            Event::ExportSubscriptions(subs_opml_name) => {
                let xml_tag = r#"<?xml version="1.0" encoding="ISO-8859-1"?>"#.to_string();
                let custom_head = Head {
                    title: Some(subs_opml_name.clone()),
                    date_created: Some(Local::now().format(OPML_DATE_FORMAT).to_string()),
                    owner_name: Some("Crab News".to_string()),
                    ..Head::default()
                };
                let custon_opml = OPML {
                    head: Some(custom_head),
                    body: model.subscriptions.body.clone(),
                    ..OPML::default()
                };
                let export_content = xml_tag + &custon_opml.to_string().unwrap();
                let _ = std::fs::write(subs_opml_name, &export_content);
            }
            Event::AddNewFolder(folder_name) => {
                let new_folder = Outline {
                    text: folder_name.clone(),
                    title: Some(folder_name.clone()),
                    ..Outline::default()
                };
                model.subscriptions.body.outlines.push(new_folder);
            }
            Event::DeleteFolder(folder_name) => {
                model
                    .subscriptions
                    .body
                    .outlines
                    .retain(|name| name.text != folder_name);
            }
            Event::RenameFolder(old_name, new_name) => {
                CrabNews::body_outlines_iter_mut(model)
                    .filter(|outline| outline.text == old_name)
                    .for_each(|folder| {
                        folder.text = new_name.clone();
                        folder.title = Some(new_name.clone());
                    });
            }
            Event::AddNewSubscription(folder_name, sub_name, sub_url) => {
                if let Some(folder_text) = folder_name {
                    CrabNews::body_outlines_iter_mut(model)
                        .filter(|outline| outline.text == folder_text)
                        .for_each(|folder| {
                            folder.add_feed(sub_name.as_str(), sub_url.as_str());
                        });
                } else {
                    model
                        .subscriptions
                        .add_feed(sub_name.as_str(), sub_url.as_str());
                }
            }
            Event::DeleteSubscription(folder_name, sub_name) => {
                if let Some(folder_text) = folder_name {
                    CrabNews::body_outlines_iter_mut(model)
                        .filter(|outline| outline.text == folder_text)
                        .for_each(|folder| folder.outlines.retain(|name| name.text != sub_name));
                } else {
                    model
                        .subscriptions
                        .body
                        .outlines
                        .retain(|name| name.text != sub_name);
                }
            }
            Event::RenameSubscription(folder_name, old_name, new_name) => {
                if let Some(folder_text) = folder_name {
                    CrabNews::body_outlines_iter_mut(model)
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
                    CrabNews::body_outlines_iter_mut(model)
                        .filter(|outline| outline.text == old_name)
                        .for_each(|sub| sub.text = new_name.clone());
                }
            }
            Event::MoveSubscriptionToFolder(subscription, old_folder, new_folder) => {
                match (old_folder, new_folder) {
                    (None, Some(folder_new)) => {
                        self.update(
                            Event::AddNewSubscription(
                                Some(folder_new),
                                subscription.text.clone(),
                                subscription.xml_url.unwrap(),
                            ),
                            model,
                            caps,
                        );
                        self.update(
                            Event::DeleteSubscription(None, subscription.text.clone()),
                            model,
                            caps,
                        )
                    }
                    (Some(folder_old), None) => {
                        self.update(
                            Event::AddNewSubscription(
                                None,
                                subscription.text.clone(),
                                subscription.xml_url.unwrap(),
                            ),
                            model,
                            caps,
                        );
                        self.update(
                            Event::DeleteSubscription(Some(folder_old), subscription.text.clone()),
                            model,
                            caps,
                        );
                    }
                    (Some(folder_old), Some(folder_new)) => {
                        self.update(
                            Event::AddNewSubscription(
                                Some(folder_new),
                                subscription.text.clone(),
                                subscription.xml_url.unwrap(),
                            ),
                            model,
                            caps,
                        );
                        self.update(
                            Event::DeleteSubscription(Some(folder_old), subscription.text.clone()),
                            model,
                            caps,
                        );
                    }
                    _ => panic!(),
                };
            }
            Event::Fetch(_) => todo!(),
        };

        caps.render.render();
    }

    fn view(&self, _model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            // subscription_folder: format!("Count is: {}", model.subscription_folder),
            // subscription_name: format!("Count is: {}", model.subscription_name),
        }
    }
}
// ANCHOR_END: impl_app
// ANCHOR_END: app

// ANCHOR: test
// TODO add all checks for dupes and so on
#[cfg(test)]
mod test {
    use super::*;
    use crux_core::testing::AppTester;

    #[test]
    fn import_subscriptions() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let subs_opml_file = "example_import.opml".to_string();
        let example_subs = r#"<opml version="2.0"><head><title>Subscriptions.opml</title><dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated><ownerName>Crab News</ownerName></head><body><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/><outline text="Group Name" title="Group Name"><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/></outline></body></opml>"#;

        let _ = app.update(Event::ImportSubscriptions(subs_opml_file), &mut model);
        let added_subs = model.subscriptions;
        let expected_subs = OPML::from_str(example_subs).unwrap();

        assert_eq!(added_subs, expected_subs);
    }

    #[test]
    fn export_subscriptions() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let date_created = Some(Local::now().format(OPML_DATE_FORMAT).to_string());
        let subs_opml_name = format!("{} - Subscriptions.opml", date_created.clone().unwrap());
        let example_subs = format!("<opml version=\"2.0\"><head><title>{}</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", subs_opml_name, date_created.unwrap());

        model.subscriptions = OPML::from_str(&example_subs).unwrap();
        let imported_content = model.subscriptions.clone();

        let _ = app.update(
            Event::ExportSubscriptions(subs_opml_name.clone()),
            &mut model,
        );

        let mut exported_file = std::fs::File::open(subs_opml_name.clone()).unwrap();
        let exported_content = OPML::from_reader(&mut exported_file).unwrap();

        assert_eq!(exported_content, imported_content);
    }

    #[test]
    fn add_new_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "Added Folder".to_string();
        let added_folder = &Outline {
            text: folder_name.to_string(),
            title: Some(folder_name.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let does_contain_folder = model.subscriptions.body.outlines.contains(added_folder);

        assert_eq!(does_contain_folder, true);
    }

    #[test]
    fn delete_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let deleted_folder = &Outline {
            text: "Deleted Folder".to_string(),
            title: Some("Deleted Folder".to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(deleted_folder.text.clone()), &mut model);
        let _ = app.update(Event::DeleteFolder(deleted_folder.text.clone()), &mut model);

        let does_contain_folder = model.subscriptions.body.outlines.contains(deleted_folder);

        assert_eq!(does_contain_folder, false);
    }

    #[test]
    fn rename_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
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

        let _ = app.update(Event::AddNewFolder(rename_folder.text.clone()), &mut model);
        let _ = app.update(
            Event::RenameFolder(rename_folder.text.clone(), expected_folder.text.clone()),
            &mut model,
        );

        let does_contain_folder = model.subscriptions.body.outlines.contains(expected_folder);

        assert_eq!(does_contain_folder, true);
    }

    #[test]
    fn add_new_subscription_to_root() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let sub_name = "New Sub Root".to_string();
        let sub_link = "https://example.com/".to_string();
        let expected_sub = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewSubscription(None, sub_name.clone(), sub_link.clone()),
            &mut model,
        );

        let does_contain_sub = model.subscriptions.body.outlines.contains(expected_sub);

        assert_eq!(does_contain_sub, true);
    }

    #[test]
    fn add_new_subscription_to_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "New Sub Folder".to_string();
        let sub_name = "Sub Name".to_string();
        let sub_link = "https://example.com/".to_string();
        let expected_sub = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                sub_name.clone(),
                sub_link.clone(),
            ),
            &mut model,
        );

        let does_contain_sub = CrabNews::body_outlines_iter_mut(&mut model)
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert_eq!(does_contain_sub, true);
    }

    #[test]
    fn delete_subscription_from_root() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let deleted_sub = &Outline {
            text: "Deleted Sub Root".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewSubscription(
                None,
                deleted_sub.text.clone(),
                deleted_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::DeleteSubscription(None, deleted_sub.text.clone()),
            &mut model,
        );

        let does_contain_sub = model.subscriptions.body.outlines.contains(deleted_sub);

        assert_eq!(does_contain_sub, false);
    }

    #[test]
    fn delete_subscription_from_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "Deleted Sub Folder".to_string();
        let deleted_sub = &Outline {
            text: "Sub Name".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                deleted_sub.text.clone(),
                deleted_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::DeleteSubscription(Some(folder_name.clone()), deleted_sub.text.clone()),
            &mut model,
        );

        let does_contain_sub = CrabNews::body_outlines_iter_mut(&mut model)
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(deleted_sub)))
            .unwrap();

        assert_eq!(does_contain_sub, false);
    }

    #[test]
    fn delete_subscription_from_folder_with_multi_subs() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "Deleted Multi Subs".to_string();
        let delete_sub = &Outline {
            text: "Deleted Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };
        let expected_sub = &Outline {
            text: "Expected Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                delete_sub.text.clone(),
                delete_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                expected_sub.text.clone(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::DeleteSubscription(Some(folder_name.clone()), delete_sub.text.clone()),
            &mut model,
        );

        let does_contain_deleted_sub = CrabNews::body_outlines_iter_mut(&mut model)
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(delete_sub)))
            .unwrap();

        let does_contain_expected_sub = CrabNews::body_outlines_iter_mut(&mut model)
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert_eq!(
            (!does_contain_deleted_sub && does_contain_expected_sub),
            true
        );
    }

    #[test]
    fn rename_subscription_in_root() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };
        let expected_sub = &Outline {
            text: "Renamed Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewSubscription(
                None,
                rename_sub.text.clone(),
                rename_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(None, rename_sub.text.clone(), expected_sub.text.clone()),
            &mut model,
        );

        let does_contain_sub = model.subscriptions.body.outlines.contains(expected_sub);

        assert_eq!(does_contain_sub, true);
    }

    #[test]
    fn rename_subscription_in_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "Renamed Sub Folder".to_string();
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };
        let expected_sub = &Outline {
            text: "Renamed Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                rename_sub.text.clone(),
                rename_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(
                Some(folder_name.clone()),
                rename_sub.text.clone(),
                expected_sub.text.clone(),
            ),
            &mut model,
        );

        let does_contain_sub = CrabNews::body_outlines_iter_mut(&mut model)
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert_eq!(does_contain_sub, true);
    }

    #[test]
    fn rename_subscription_in_folder_with_multi_subs() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "Renamed Multi Sub Folder".to_string();
        let untouched_sub = &Outline {
            text: "Untouched Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };
        let rename_sub = &Outline {
            text: "Old Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };
        let expected_sub = &Outline {
            text: "Renamed Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                untouched_sub.text.clone(),
                untouched_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                expected_sub.text.clone(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(
                Some(folder_name.clone()),
                rename_sub.text.clone(),
                expected_sub.text.clone(),
            ),
            &mut model,
        );

        let does_contain_untouched_sub = CrabNews::body_outlines_iter_mut(&mut model)
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(untouched_sub)))
            .unwrap();

        let does_contain_expected_sub = CrabNews::body_outlines_iter_mut(&mut model)
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert_eq!(
            (does_contain_untouched_sub && does_contain_expected_sub),
            true
        );
    }

    #[test]
    fn move_subscription_from_root_to_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "Move Sub To Folder".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            title: Some("Moved Sub".to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(
                None,
                expected_sub.text.clone(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscriptionToFolder(expected_sub.clone(), None, Some(folder_name.clone())),
            &mut model,
        );

        let does_contain_sub = CrabNews::body_outlines_iter_mut(&mut model)
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert_eq!(does_contain_sub, true);
    }

    #[test]
    fn move_subscription_from_folder_to_root() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "Move Sub To Root".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            title: Some("Moved Sub".to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                expected_sub.text.clone(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscriptionToFolder(expected_sub.clone(), Some(folder_name.clone()), None),
            &mut model,
        );

        let does_contain_sub = model.subscriptions.body.outlines.contains(expected_sub);

        assert_eq!(does_contain_sub, true);
    }

    #[test]
    fn move_subscription_from_folder_to_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_one = "Folder One".to_string();
        let folder_two = "Folder Two".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            title: Some("Moved Sub".to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_one.clone()), &mut model);
        let _ = app.update(Event::AddNewFolder(folder_two.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_one.clone()),
                expected_sub.text.clone(),
                expected_sub.xml_url.clone().unwrap().clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscriptionToFolder(
                expected_sub.clone(),
                Some(folder_one.clone()),
                Some(folder_two.clone()),
            ),
            &mut model,
        );

        let does_contain_sub = CrabNews::body_outlines_iter_mut(&mut model)
            .filter(|outline| outline.text == folder_two)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert_eq!(does_contain_sub, true);
    }
}
// ANCHOR_END: test
