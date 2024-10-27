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
    MoveSubscriptionToFolder(OldFolder, NewFolder, SubscriptionName),

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
                    .for_each(|sub| {
                        sub.text = new_name.clone();
                        sub.title = Some(new_name.clone());
                    });
            }
            Event::AddNewSubscription(folder_name, sub_name, sub_url) => {
                if let Some(folder_text) = folder_name {
                    CrabNews::body_outlines_iter_mut(model)
                        .filter(|outline| outline.text == folder_text)
                        .for_each(|sub| {
                            sub.add_feed(sub_name.as_str(), sub_url.as_str());
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
                        .for_each(|sub| sub.outlines.retain(|name| name.text != sub_name));
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
            Event::MoveSubscriptionToFolder(old_folder, new_folder, sub_name) => {}
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
        let folder_name = "dada".to_string();
        let expected_folder = &Outline {
            text: folder_name.to_string(),
            title: Some(folder_name.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let does_contain_folder = model.subscriptions.body.outlines.contains(expected_folder);

        assert_eq!(does_contain_folder, true);
    }

    #[test]
    fn delete_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "dada".to_string();
        let expected_folder = &Outline {
            text: folder_name.to_string(),
            title: Some(folder_name.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(Event::DeleteFolder(folder_name.clone()), &mut model);

        let does_contain_folder = model.subscriptions.body.outlines.contains(expected_folder);

        assert_eq!(does_contain_folder, false);
    }

    #[test]
    fn rename_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let old_name = "dada".to_string();
        let new_name = "tada".to_string();
        let expected_folder = &Outline {
            text: new_name.to_string(),
            title: Some(new_name.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(old_name.clone()), &mut model);
        let _ = app.update(
            Event::RenameFolder(old_name.clone(), new_name.clone()),
            &mut model,
        );

        let does_contain_folder = model.subscriptions.body.outlines.contains(expected_folder);

        assert_eq!(does_contain_folder, true);
    }

    #[test]
    fn add_new_subscription_to_root() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let sub_name = "Feed Name".to_string();
        let sub_url = "https://example.com/".to_string();
        let expected_feed = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewSubscription(None, sub_name.clone(), sub_url.clone()),
            &mut model,
        );

        let does_contain_feed = model.subscriptions.body.outlines.contains(expected_feed);

        assert_eq!(does_contain_feed, true);
    }

    #[test]
    fn add_new_subscription_to_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "dada".to_string();
        let sub_name = "Feed Name".to_string();
        let sub_url = "https://example.com/".to_string();
        let expected_feed = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(Some(folder_name.clone()), sub_name.clone(), sub_url.clone()),
            &mut model,
        );

        let does_contain_feed = model
            .subscriptions
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_feed)))
            .unwrap();

        assert_eq!(does_contain_feed, true);
    }

    #[test]
    fn delete_subscription_from_root() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let sub_name = "Feed Name".to_string();
        let sub_url = "https://example.com/".to_string();
        let expected_feed = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewSubscription(None, sub_name.clone(), sub_url.clone()),
            &mut model,
        );
        let _ = app.update(
            Event::DeleteSubscription(None, sub_name.clone()),
            &mut model,
        );

        let does_contain_feed = model.subscriptions.body.outlines.contains(expected_feed);

        assert_eq!(does_contain_feed, false);
    }

    #[test]
    fn delete_subscription_from_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "dada".to_string();
        let sub_name = "Feed Name".to_string();
        let sub_url = "https://example.com/".to_string();
        let expected_feed = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(Some(folder_name.clone()), sub_name.clone(), sub_url.clone()),
            &mut model,
        );
        let _ = app.update(
            Event::DeleteSubscription(Some(folder_name.clone()), sub_name.clone()),
            &mut model,
        );

        let does_contain_feed = model
            .subscriptions
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_feed)))
            .unwrap();

        assert_eq!(does_contain_feed, false);
    }

    #[test]
    fn delete_subscription_from_folder_with_multi_subs() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "dada".to_string();
        let sub_one_name = "Feed One Name".to_string();
        let sub_one_url = "https://example.com/".to_string();
        let sub_two_name = "Feed Two Name".to_string();
        let sub_two_url = "https://example.com/".to_string();
        let deleted_feed = &Outline {
            text: sub_one_name.to_string(),
            xml_url: Some(sub_one_url.to_string()),
            ..Outline::default()
        };
        let expected_feed = &Outline {
            text: sub_two_name.to_string(),
            xml_url: Some(sub_two_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                sub_one_name.clone(),
                sub_one_url.clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                sub_two_name.clone(),
                sub_two_url.clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::DeleteSubscription(Some(folder_name.clone()), sub_one_name.clone()),
            &mut model,
        );

        let does_contain_feed_one = model
            .subscriptions
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(deleted_feed)))
            .unwrap();

        let does_contain_feed_two = model
            .subscriptions
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_feed)))
            .unwrap();

        assert_eq!((does_contain_feed_two && !does_contain_feed_one), true);
    }

    #[test]
    fn rename_subscription_in_root() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let old_name = "Feed Name".to_string();
        let sub_url = "https://example.com/".to_string();
        let new_name = "New Name".to_string();
        let expected_feed = &Outline {
            text: new_name.to_string(),
            xml_url: Some(sub_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(
            Event::AddNewSubscription(None, old_name.clone(), sub_url.clone()),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(None, old_name.clone(), new_name.clone()),
            &mut model,
        );

        let does_contain_feed = model.subscriptions.body.outlines.contains(expected_feed);

        assert_eq!(does_contain_feed, true);
    }

    #[test]
    fn rename_subscription_in_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let old_name = "Feed Name".to_string();
        let sub_url = "https://example.com/".to_string();
        let new_name = "New Name".to_string();
        let folder_name = "dada".to_string();
        let expected_feed = &Outline {
            text: new_name.to_string(),
            xml_url: Some(sub_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(Some(folder_name.clone()), old_name.clone(), sub_url.clone()),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(
                Some(folder_name.clone()),
                old_name.clone(),
                new_name.clone(),
            ),
            &mut model,
        );

        let does_contain_feed = model
            .subscriptions
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_feed)))
            .unwrap();

        assert_eq!(does_contain_feed, true);
    }

    #[test]
    fn rename_subscription_in_folder_with_multi_subs() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "dada".to_string();
        let sub_one_name = "Feed One Name".to_string();
        let sub_one_url = "https://example.com/".to_string();
        let sub_two_name = "Feed Two Name".to_string();
        let sub_two_url = "https://exampleaaaa.com/".to_string();
        let new_name = "Renamed Name".to_string();
        let expected_feed = &Outline {
            text: new_name.to_string(),
            xml_url: Some(sub_one_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                sub_one_name.clone(),
                sub_one_url.clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                sub_two_name.clone(),
                sub_two_url.clone(),
            ),
            &mut model,
        );
        let _ = app.update(
            Event::RenameSubscription(
                Some(folder_name.clone()),
                sub_one_name.clone(),
                new_name.clone(),
            ),
            &mut model,
        );

        let does_contain_feed = model
            .subscriptions
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_feed)))
            .unwrap();

        assert_eq!(does_contain_feed, true);
    }

    #[test]
    fn move_subscription_from_root_to_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let sub_name = "Feed Name".to_string();
        let sub_url = "https://example.com/".to_string();
        let folder_name = "dada".to_string();
        let expected_feed = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(None, sub_name.clone(), sub_url.clone()),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscriptionToFolder(None, Some(folder_name.clone()), sub_name.clone()),
            &mut model,
        );

        let does_contain_feed = model
            .subscriptions
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_feed)))
            .unwrap();

        assert_eq!(does_contain_feed, true);
    }

    #[test]
    fn move_subscription_from_folder_to_root() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "dada".to_string();
        let sub_name = "Feed Name".to_string();
        let sub_url = "https://example.com/".to_string();
        let expected_feed = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(Some(folder_name.clone()), sub_name.clone(), sub_url.clone()),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscriptionToFolder(Some(folder_name.clone()), None, sub_name.clone()),
            &mut model,
        );

        let does_contain_feed = model.subscriptions.body.outlines.contains(expected_feed);

        assert_eq!(does_contain_feed, true);
    }

    #[test]
    fn move_subscription_from_folder_to_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_one = "Folder One".to_string();
        let folder_two = "Folder Two".to_string();
        let sub_name = "Feed Name".to_string();
        let sub_url = "https://example.com/".to_string();
        let expected_feed = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_url.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_one.clone()), &mut model);
        let _ = app.update(Event::AddNewFolder(folder_two.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(Some(folder_one.clone()), sub_name.clone(), sub_url.clone()),
            &mut model,
        );
        let _ = app.update(
            Event::MoveSubscriptionToFolder(
                Some(folder_one.clone()),
                Some(folder_two.clone()),
                sub_name.clone(),
            ),
            &mut model,
        );

        let does_contain_feed = model
            .subscriptions
            .body
            .outlines
            .iter()
            .filter(|outline| outline.text == folder_two)
            .find_map(|folder| Some(folder.outlines.contains(expected_feed)))
            .unwrap();

        assert_eq!(does_contain_feed, true);
    }
}
// ANCHOR_END: test
