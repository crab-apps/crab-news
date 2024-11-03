// ANCHOR: app
use crux_core::{render::Render, App};
use feed_rs::model::Feed;
use serde::{Deserialize, Serialize};
// use crux_http::Http;
// use url::Url;

mod subscriptions;
pub use subscriptions::{
    FolderName, NewFolder, NewName, OldFolder, OldName, OpmlFile, OpmlName, OutlineError,
    Subscription, SubscriptionName, SubscriptionURL, Subscriptions,
};

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

// ANCHOR: model
#[derive(Default, Serialize)]
pub struct Model {
    subscriptions: Subscriptions,
    outline_error: OutlineError,
}

// ANCHOR: view model
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ViewModel {
    // pub subscription_folder: String,
    // pub subscription_name: String,
    // subscriptions: Subscriptions,
    pub outline_error: OutlineError,
}
// ANCHOR_END: view model
// ANCHOR_END: model

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Default)]
pub struct CrabNews;

// ANCHOR: traits
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
                match Subscriptions::import(&mut model.subscriptions, subs_opml_file) {
                    Ok(subscriptions) => subscriptions,
                    Err(e) => {
                        return model.outline_error = OutlineError {
                            title: "Import Error".to_string(),
                            message: e.to_string(),
                        }
                    }
                };
                ()
            }
            Event::ExportSubscriptions(subs_opml_name) => {
                Subscriptions::export(&model.subscriptions, subs_opml_name);
            }
            Event::AddNewFolder(folder_name) => {
                match Subscriptions::add_folder(&mut model.subscriptions, folder_name) {
                    Ok(subscriptions) => subscriptions,
                    Err(e) => {
                        return model.outline_error = OutlineError {
                            title: "New Folder Error".to_string(),
                            message: e.to_string(),
                        }
                    }
                };
                ()
            }
            Event::DeleteFolder(folder_name) => {
                Subscriptions::delete_folder(&mut model.subscriptions, folder_name);
            }
            Event::RenameFolder(old_folder_name, new_folder_name) => {
                match Subscriptions::rename_folder(
                    &mut model.subscriptions,
                    old_folder_name,
                    new_folder_name,
                ) {
                    Ok(subscriptions) => subscriptions,
                    Err(e) => {
                        return model.outline_error = OutlineError {
                            title: "Rename Folder Error".to_string(),
                            message: e.to_string(),
                        }
                    }
                };
                ()
            }
            Event::AddNewSubscription(folder_name, sub_name, sub_url) => {
                match Subscriptions::add_subscription(
                    &mut model.subscriptions,
                    folder_name,
                    sub_name,
                    sub_url,
                ) {
                    Ok(subscriptions) => subscriptions,
                    Err(e) => {
                        return model.outline_error = OutlineError {
                            title: "Subscription Error".to_string(),
                            message: e.to_string(),
                        }
                    }
                };
                ()
            }
            Event::DeleteSubscription(folder_name, sub_name) => {
                Subscriptions::delete_subscription(&mut model.subscriptions, folder_name, sub_name);
            }
            Event::RenameSubscription(folder_name, old_name, new_name) => {
                match Subscriptions::rename_subscription(
                    &mut model.subscriptions,
                    folder_name,
                    old_name,
                    new_name,
                ) {
                    Ok(subscriptions) => subscriptions,
                    Err(e) => {
                        return model.outline_error = OutlineError {
                            title: "Subscription Error".to_string(),
                            message: e.to_string(),
                        }
                    }
                };
                ()
            }
            Event::MoveSubscriptionToFolder(subscription, old_folder, new_folder) => {
                Subscriptions::move_subscription(
                    &mut model.subscriptions,
                    subscription,
                    old_folder,
                    new_folder,
                );
            }
            Event::Fetch(_) => todo!(),
        };

        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            outline_error: model.outline_error.clone(),
            // subscription_folder: format!("Count is: {}", model.subscription_folder),
            // subscription_name: format!("Count is: {}", model.subscription_name),
            // subscriptions: format!("Count is: {:?}", model.subscriptions),
        }
    }
}
// ANCHOR_END: impl_app
// ANCHOR_END: app

// ANCHOR: test
// TODO add all checks outline_error: model.outline_error.clone(),
#[cfg(test)]
mod test {
    use super::*;
    use chrono::prelude::Local;
    use crux_core::testing::AppTester;
    use opml::{Outline, OPML};

    #[test]
    fn import_subscriptions() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let subs_opml_file = "example_import.opml".to_string();
        let example_subs = r#"<opml version="2.0"><head><title>Subscriptions.opml</title><dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated><ownerName>Crab News</ownerName></head><body><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/><outline text="Group Name" title="Group Name"><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/></outline></body></opml>"#;

        let _ = app.update(Event::ImportSubscriptions(subs_opml_file), &mut model);
        let added_subs = model.subscriptions;
        let expected_subs = Subscriptions {
            opml: OPML::from_str(example_subs).unwrap(),
        };

        assert_eq!(added_subs, expected_subs);
    }

    #[test]
    fn fail_import_for_invalid_xml() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let subs_opml_file = "invalid_xml.opml".to_string();

        let _ = app.update(Event::ImportSubscriptions(subs_opml_file), &mut model);
        let actual_error = model.outline_error.message;
        let expected_error = "Failed to process XML file";

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn fail_import_for_invalid_opml_version() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let subs_opml_file = "invalid_opml_version.opml".to_string();

        let _ = app.update(Event::ImportSubscriptions(subs_opml_file), &mut model);
        let actual_error = model.outline_error.message;
        let expected_error = "Unsupported OPML version: \"0.1\"";

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn export_subscriptions() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let date_created = Some(Local::now().format("%Y - %a %b %e %T").to_string());
        let subs_opml_name = format!("{} - Subscriptions.opml", date_created.clone().unwrap());
        let example_subs = format!("<opml version=\"2.0\"><head><title>{}</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", subs_opml_name, date_created.unwrap());

        model.subscriptions = Subscriptions {
            opml: OPML::from_str(&example_subs).unwrap(),
        };
        let imported_content = model.subscriptions.clone();

        let _ = app.update(
            Event::ExportSubscriptions(subs_opml_name.clone()),
            &mut model,
        );

        let mut exported_file = std::fs::File::open(subs_opml_name.clone()).unwrap();
        let exported_content = Subscriptions {
            opml: OPML::from_reader(&mut exported_file).unwrap(),
        };

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
        let does_contain_folder = model
            .subscriptions
            .opml
            .body
            .outlines
            .contains(added_folder);

        assert_eq!(does_contain_folder, true);
    }

    #[test]
    fn fail_add_new_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "Added Folder".to_string();

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let actual_error = model.outline_error.message;
        let expected_error = format!(
            "Cannot add new folder \"{}\". It already exists.",
            folder_name
        );

        assert_eq!(actual_error, expected_error);
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

        let does_contain_folder = model
            .subscriptions
            .opml
            .body
            .outlines
            .contains(deleted_folder);

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

        let does_contain_folder = model
            .subscriptions
            .opml
            .body
            .outlines
            .contains(expected_folder);

        assert_eq!(does_contain_folder, true);
    }

    #[test]
    fn fail_rename_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let test_folder = &Outline {
            text: "Expected Folder".to_string(),
            title: Some("Expected Folder".to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(test_folder.text.clone()), &mut model);
        let _ = app.update(
            Event::RenameFolder(test_folder.text.clone(), test_folder.text.clone()),
            &mut model,
        );
        let actual_error = model.outline_error.message;
        let expected_error = format!(
            "Cannot rename folder \"{}\" to \"{}\". It already exists.",
            test_folder.text.to_string(),
            test_folder.text.to_string()
        );

        assert_eq!(actual_error, expected_error);
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

        let does_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .contains(expected_sub);

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

        let does_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert_eq!(does_contain_sub, true);
    }

    #[test]
    fn fail_add_new_subscription_to_root() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "New Sub Folder".to_string();
        let sub_name = "Sub Name".to_string();
        let sub_link = "https://example.com/".to_string();
        let test_subscription = &Outline {
            text: sub_name.to_string(),
            xml_url: Some(sub_link.to_string()),
            ..Outline::default()
        };

        let _ = app.update(Event::AddNewFolder(folder_name.clone()), &mut model);
        let _ = app.update(
            Event::AddNewSubscription(None, sub_name.clone(), sub_link.clone()),
            &mut model,
        );
        let _ = app.update(
            Event::AddNewSubscription(None, sub_name.clone(), sub_link.clone()),
            &mut model,
        );
        let actual_error = model.outline_error.message;
        let expected_error = format!(
            "Cannot add new subscription \"{}\". You are already subscribed.",
            test_subscription.text.to_string()
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn fail_add_new_subscription_to_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "New Sub Folder".to_string();
        let sub_name = "Sub Name".to_string();
        let sub_link = "https://example.com/".to_string();
        let test_subscription = &Outline {
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
        let _ = app.update(
            Event::AddNewSubscription(
                Some(folder_name.clone()),
                sub_name.clone(),
                sub_link.clone(),
            ),
            &mut model,
        );
        let actual_error = model.outline_error.message;
        let expected_error = format!(
            "Cannot add new subscription \"{}\". You are already subscribed.",
            test_subscription.text.to_string()
        );

        assert_eq!(actual_error, expected_error);
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

        let does_contain_sub = model.subscriptions.opml.body.outlines.contains(deleted_sub);

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

        let does_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
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

        let does_contain_deleted_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(delete_sub)))
            .unwrap();

        let does_contain_expected_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
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

        let does_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .contains(expected_sub);

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

        let does_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
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

        let does_contain_untouched_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(untouched_sub)))
            .unwrap();

        let does_contain_expected_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
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
            xml_url: Some("https://example.com/".to_string()),
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

        let does_root_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .contains(expected_sub);

        let does_folder_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert_eq!((!does_root_contain_sub && does_folder_contain_sub), true);
    }

    #[test]
    fn move_subscription_from_folder_to_root() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_name = "Move Sub To Root".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
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

        let does_root_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .contains(expected_sub);

        let does_folder_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
            .filter(|outline| outline.text == folder_name)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert_eq!((does_root_contain_sub && !does_folder_contain_sub), true);
    }

    #[test]
    fn move_subscription_from_folder_to_folder() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let folder_one = "Folder One".to_string();
        let folder_two = "Folder Two".to_string();
        let expected_sub = &Outline {
            text: "Moved Sub".to_string(),
            xml_url: Some("https://example.com/".to_string()),
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

        let does_folder_one_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
            .filter(|outline| outline.text == folder_one)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        let does_folder_two_contain_sub = model
            .subscriptions
            .opml
            .body
            .outlines
            .iter_mut()
            .filter(|outline| outline.text == folder_two)
            .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
            .unwrap();

        assert_eq!(
            (!does_folder_one_contain_sub && does_folder_two_contain_sub),
            true
        );
    }
}
// ANCHOR_END: test
