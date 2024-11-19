// ANCHOR: app
// ANCHOR: imports
use crux_core::{render::Render, App};
use crux_http::Http;
use serde::{Deserialize, Serialize};
// use url::Url;

mod account;
pub use account::{Account, AccountType};

// NOTE - crate: https://crates.io/crates/opml
// to deal with subscriptions and outlines:
mod subscriptions;
pub use subscriptions::{
    FolderName, NewFolder, NewName, OldFolder, OldLink, OldName, OpmlFile, OpmlName, Subscription,
    SubscriptionLink, SubscriptionTitle,
};

// NOTE - crate: https://crates.io/crates/feed-rs
// to deal with feeds data *after* subscribtions.
// to deal with shell data to display "news" in entry and content columns.
// mod feeds;
// pub use feeds::Feeds;
// ANCHOR_END: imports

// ANCHOR: events
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    // EVENTS FROM THE SHELL
    AddAccount(AccountType),
    DeleteAccount(AccountType),
    // ImportSubscriptions(OpmlFile),
    // ExportSubscriptions(OpmlName),
    // AddNewFolder(FolderName),
    // DeleteFolder(FolderName),
    // RenameFolder(OldName, NewName),
    // AddNewSubscription(Option<FolderName>, SubscriptionTitle, SubscriptionLink),
    // DeleteSubscription(Option<FolderName>, SubscriptionTitle),
    // RenameSubscription(Option<FolderName>, OldName, OldLink, NewName),
    // MoveSubscriptionToFolder(Subscription, OldFolder, NewFolder),
    // Get,
    // EVENTS LOCAL TO THE CORE
    // #[serde(skip)]
    // Set(crux_http::Result<crux_http::Response<Feed>>),
}
// ANCHOR_END: events

// ANCHOR: model
// NOTE feed-rs has NO Serialize Deserialize
#[derive(Default, Serialize)]
pub struct Model {
    notification: Notification,
    accounts: Vec<Account>,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Notification {
    pub title: String,
    pub message: String,
}

// ANCHOR: view model
// NOTE feed-rs has NO Serialize Deserialize
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ViewModel {
    pub notification: Notification,
}
// ANCHOR_END: view model
// ANCHOR_END: model

// ANCHOR: capabilities
#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    pub render: Render<Event>,
    pub http: Http<Event>,
}
// ANCHOR_END: capabilities

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
            Event::AddAccount(account_type) => {
                match Account::add_account(&model.accounts, account_type) {
                    Ok(accts) => model.accounts = accts,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Account Error".to_string(),
                            message: err.to_string(),
                        }
                    }
                }
            }
            Event::DeleteAccount(account_type) => {
                model.accounts = Account::delete(&model.accounts, account_type);
            } // Event::ImportSubscriptions(subs_opml_file) => {
              //     match Subscriptions::import(&model.subscriptions, subs_opml_file) {
              //         // TODO on duplicates, prompt user for merge or replace
              //         Ok(subs) => model.subscriptions = subs,
              //         Err(err) => {
              //             return model.notification = Notification {
              //                 title: "Import Error".to_string(),
              //                 message: err.to_string(),
              //             }
              //         }
              //     };
              //     ()
              // }
              // Event::ExportSubscriptions(subs_opml_name) => {
              //     match Subscriptions::export(&model.subscriptions, subs_opml_name) {
              //         Ok(success) => {
              //             return model.notification = Notification {
              //                 title: "Subscriptions Exported".to_string(),
              //                 message: success.to_string(),
              //             }
              //         }
              //         Err(err) => {
              //             // TODO once shell is implemented, check failures
              //             return model.notification = Notification {
              //                 title: "Export Error".to_string(),
              //                 message: err.to_string(),
              //             };
              //         }
              //     };
              // }
              // Event::AddNewFolder(folder_name) => {
              //     match Subscriptions::add_folder(&model.subscriptions, folder_name) {
              //         Ok(subs) => model.subscriptions = subs,
              //         Err(err) => {
              //             return model.notification = Notification {
              //                 title: "New Folder Error".to_string(),
              //                 message: err.to_string(),
              //             }
              //         }
              //     };
              //     ()
              // }
              // Event::DeleteFolder(folder_name) => {
              //     model.subscriptions =
              //         Subscriptions::delete_folder(&model.subscriptions, folder_name);
              // }
              // Event::RenameFolder(old_folder_name, new_folder_name) => {
              //     match Subscriptions::rename_folder(
              //         &model.subscriptions,
              //         old_folder_name,
              //         new_folder_name,
              //     ) {
              //         Ok(subs) => model.subscriptions = subs,
              //         Err(err) => {
              //             return model.notification = Notification {
              //                 title: "Rename Folder Error".to_string(),
              //                 message: err.to_string(),
              //             }
              //         }
              //     };
              //     ()
              // }
              // Event::AddNewSubscription(folder_name, sub_title, sub_link) => {
              //     match Subscriptions::add_subscription(
              //         &model.subscriptions,
              //         folder_name,
              //         sub_title,
              //         sub_link,
              //     ) {
              //         Ok(subs) => model.subscriptions = subs,
              //         Err(err) => {
              //             return model.notification = Notification {
              //                 title: "Subscription Error".to_string(),
              //                 message: err.to_string(),
              //             }
              //         }
              //     };
              //     ()
              // }
              // Event::DeleteSubscription(folder_name, sub_name) => {
              //     model.subscriptions =
              //         Subscriptions::delete_subscription(&model.subscriptions, folder_name, sub_name);
              // }
              // Event::RenameSubscription(folder_name, old_title, old_link, new_name) => {
              //     match Subscriptions::rename_subscription(
              //         &model.subscriptions,
              //         folder_name,
              //         old_title,
              //         old_link,
              //         new_name,
              //     ) {
              //         Ok(subs) => model.subscriptions = subs,
              //         Err(err) => {
              //             return model.notification = Notification {
              //                 title: "Subscription Error".to_string(),
              //                 message: err.to_string(),
              //             }
              //         }
              //     };
              //     ()
              // }
              // Event::MoveSubscriptionToFolder(subscription, old_folder, new_folder) => {
              //     match Subscriptions::move_subscription(
              //         &model.subscriptions,
              //         subscription,
              //         old_folder,
              //         new_folder,
              //     ) {
              //         Ok(subs) => model.subscriptions = subs,
              //         Err(err) => {
              //             return model.notification = Notification {
              //                 title: "Subscription Error".to_string(),
              //                 message: err.to_string(),
              //             }
              //         }
              //     };
              //     ()
              // }

              // Event::Get => {
              //     caps.http.get(TEST_FEED_URL).send(Event::Set);
              // }
              // Event::Set(Ok(mut response)) => {
              //     let count = response.take_body().unwrap();
              //     // self.update(Event::Update(count), model, caps);
              // }
              // Event::Set(Err(e)) => {
              //     panic!("Oh no something went wrong: {e:?}");
              // }
        };

        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            notification: model.notification.clone(),
            // subscriptions: model.subscriptions.clone(),
            // subscription_folder: model.subscription_folder.to_string(),
            // subscription_title: model.subscription_title.to_string(),
            // subscription_link: model.subscription_link.to_string(),
            // feeds: model.feeds.clone(),
        }
    }
}
// ANCHOR_END: impl_app
// ANCHOR_END: app

// ANCHOR: test

#[cfg(test)]
mod accounts {
    use super::*;
    use crate::{CrabNews, Event, Model};
    use crux_core::testing::AppTester;
    use opml::OPML;
    use subscriptions::Subscriptions;
    use uuid::Uuid;

    #[test]
    fn add_new_local_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let added_account = Account {
            id: Uuid::new_v4(),
            name: "On Device".to_string(),
            subs: Subscriptions {
                opml: OPML::default(),
            },
        };

        let _ = app.update(Event::AddAccount(AccountType::Local), &mut model);
        let does_contain_account = model.accounts.contains(&added_account);

        assert_eq!(does_contain_account, true);
    }

    #[test]
    fn fail_new_local_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_name = "On Device".to_string();

        let _ = app.update(Event::AddAccount(AccountType::Local), &mut model);
        let _ = app.update(Event::AddAccount(AccountType::Local), &mut model);

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot add account \"{}\". It already exists.",
            account_name
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn delete_local_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let deleted_account = Account {
            id: Uuid::new_v4(),
            name: "On Device".to_string(),
            subs: Subscriptions {
                opml: OPML::default(),
            },
        };

        let _ = app.update(Event::AddAccount(AccountType::Local), &mut model);
        let _ = app.update(Event::DeleteAccount(AccountType::Local), &mut model);

        let does_contain_account = model.accounts.contains(&deleted_account);

        assert_eq!(does_contain_account, false);
    }

    #[test]
    fn add_new_native_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let added_account = Account {
            id: Uuid::new_v4(),
            name: "iCloud".to_string(),
            subs: Subscriptions {
                opml: OPML::default(),
            },
        };

        let _ = app.update(Event::AddAccount(AccountType::Apple), &mut model);
        let does_contain_account = model.accounts.contains(&added_account);

        assert_eq!(does_contain_account, true);
    }

    #[test]
    fn fail_new_native_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let account_name = "iCloud".to_string();

        let _ = app.update(Event::AddAccount(AccountType::Apple), &mut model);
        let _ = app.update(Event::AddAccount(AccountType::Apple), &mut model);

        let actual_error = model.notification.message;
        let expected_error = format!(
            "Cannot add account \"{}\". It already exists.",
            account_name
        );

        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn delete_native_account() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let deleted_account = Account {
            id: Uuid::new_v4(),
            name: "iCloud".to_string(),
            subs: Subscriptions {
                opml: OPML::default(),
            },
        };

        let _ = app.update(Event::AddAccount(AccountType::Apple), &mut model);
        let _ = app.update(Event::DeleteAccount(AccountType::Apple), &mut model);

        let does_contain_account = model.accounts.contains(&deleted_account);

        assert_eq!(does_contain_account, false);
    }
}

// #[cfg(test)]
// mod import_export {
//     use super::*;
//     use crate::{CrabNews, Event, Model};
//     use chrono::prelude::Local;
//     use crux_core::testing::AppTester;
//     use opml::OPML;

//     #[test]
//     fn import_subscriptions() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model = Model::default();
//         let subs_opml_file = "example_import.opml".to_string();
//         let example_subs = r#"<opml version="2.0"><head><title>Subscriptions.opml</title><dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated><ownerName>Crab News</ownerName></head><body><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/><outline text="Group Name" title="Group Name"><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/></outline></body></opml>"#;

//         let _ = app.update(Event::ImportSubscriptions(subs_opml_file), &mut model);
//         let added_subs = model.subscriptions;
//         let expected_subs = Subscriptions {
//             opml: OPML::from_str(example_subs).unwrap(),
//         };

//         assert_eq!(added_subs, expected_subs);
//     }

//     #[test]
//     fn fail_import_for_invalid_xml() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model = Model::default();
//         let subs_opml_file = "invalid_xml.opml".to_string();

//         let _ = app.update(Event::ImportSubscriptions(subs_opml_file), &mut model);
//         let actual_error = model.notification.message;
//         let expected_error = "Failed to process XML file";

//         assert_eq!(actual_error, expected_error);
//     }

//     #[test]
//     fn fail_import_for_invalid_opml_version() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model = Model::default();
//         let subs_opml_file = "invalid_opml_version.opml".to_string();

//         let _ = app.update(Event::ImportSubscriptions(subs_opml_file), &mut model);
//         let actual_error = model.notification.message;
//         let expected_error = "Unsupported OPML version: \"0.1\"";

//         assert_eq!(actual_error, expected_error);
//     }

//     #[test]
//     fn export_subscriptions() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let date_created = Some(Local::now().format("%Y - %a %b %e %T").to_string());
//         let subs_opml_name = "Subscriptions.opml".to_string();
//         let example_subs = format!("<opml version=\"2.0\"><head><title>{}</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName><ownerId>https://github.com/crab-apps/crab-news</ownerId></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", subs_opml_name, date_created.unwrap());

//         model.subscriptions = Subscriptions {
//             opml: OPML::from_str(&example_subs).unwrap(),
//         };
//         let imported_content = model.subscriptions.clone();

//         let _ = app.update(
//             Event::ExportSubscriptions(subs_opml_name.to_string()),
//             &mut model,
//         );

//         // TODO use proper Shell/WASM/crate functionality to File operations
//         let mut exported_file = std::fs::File::open(subs_opml_name.to_string()).unwrap();
//         let exported_content = Subscriptions {
//             opml: OPML::from_reader(&mut exported_file).unwrap(),
//         };

//         assert_eq!(exported_content, imported_content);
//     }

//     #[test]
//     fn export_subscriptions_notification() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let date_created = Some(Local::now().format("%Y - %a %b %e %T").to_string());
//         let subs_opml_name = "Subscriptions.opml".to_string();
//         let example_subs = format!("<opml version=\"2.0\"><head><title>{}</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName><ownerId>https://github.com/crab-apps/crab-news</ownerId></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", subs_opml_name, date_created.unwrap());

//         model.subscriptions = Subscriptions {
//             opml: OPML::from_str(&example_subs).unwrap(),
//         };

//         let _ = app.update(
//             Event::ExportSubscriptions(subs_opml_name.to_string()),
//             &mut model,
//         );

//         let actual_error = model.notification.message;
//         let expected_error = "Subscriptions successfully exported";

//         assert_eq!(actual_error, expected_error);
//     }

//     // TODO once shell is implemented, check failures
//     // #[test]
//     // fn fail_export_subscriptions() {
//     //     let app = AppTester::<CrabNews, _>::default();
//     //     let mut model: Model = Model::default();
//     //     let date_created = Some(Local::now().format("%Y - %a %b %e %T").to_string());
//     //     let subs_opml_name = format!("{} - Subscriptions.opml", date_created.clone().unwrap());
//     //     let example_subs = format!("<opml version=\"2.0\"><head><title>{}</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName><ownerId>https://github.com/crab-apps/crab-news</ownerId></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", subs_opml_name, date_created.unwrap());

//     //     model.subscriptions = Subscriptions {
//     //         opml: OPML::from_str(&example_subs).unwrap(),
//     //     };
//     //     let imported_content = model.subscriptions.clone();

//     //     let _ = app.update(
//     //         Event::ExportSubscriptions(subs_opml_name.clone()),
//     //         &mut model,
//     //     );

//     //     // TODO use proper Shell/WASM/crate functionality to File operations
//     //     let mut exported_file = std::fs::File::open(subs_opml_name.clone()).unwrap();
//     //     let exported_content = Subscriptions {
//     //         opml: OPML::from_reader(&mut exported_file).unwrap(),
//     //     };

//     //     assert_eq!(exported_content, imported_content);
// }

// #[cfg(test)]
// mod folder {
//     use super::*;
//     use crate::{CrabNews, Event, Model};
//     use crux_core::testing::AppTester;
//     use opml::Outline;

//     #[test]
//     fn add_new_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Added Folder".to_string();
//         let added_folder = &Outline {
//             text: folder_name.to_string(),
//             title: Some(folder_name.to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let does_contain_folder = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .contains(added_folder);

//         assert_eq!(does_contain_folder, true);
//     }

//     #[test]
//     fn add_two_new_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name_one = "Added Folder Ome".to_string();
//         let folder_name_two = "Added Folder Two".to_string();
//         let added_folder_one = &Outline {
//             text: folder_name_one.to_string(),
//             title: Some(folder_name_one.to_string()),
//             ..Outline::default()
//         };
//         let added_folder_two = &Outline {
//             text: folder_name_two.to_string(),
//             title: Some(folder_name_two.to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name_one.to_string()), &mut model);
//         let _ = app.update(Event::AddNewFolder(folder_name_two.to_string()), &mut model);
//         let does_contain_folder_one = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .contains(added_folder_one);
//         let does_contain_folder_two = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .contains(added_folder_two);

//         assert_eq!((does_contain_folder_one && does_contain_folder_two), true);
//     }

//     #[test]
//     fn fail_add_new_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Added Folder".to_string();

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let actual_error = model.notification.message;
//         let expected_error = format!(
//             "Cannot add new folder \"{}\". It already exists.",
//             folder_name
//         );

//         assert_eq!(actual_error, expected_error);
//     }

//     #[test]
//     fn delete_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let deleted_folder = &Outline {
//             text: "Deleted Folder".to_string(),
//             title: Some("Deleted Folder".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(
//             Event::AddNewFolder(deleted_folder.text.to_string()),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::DeleteFolder(deleted_folder.text.to_string()),
//             &mut model,
//         );

//         let does_contain_folder = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .contains(deleted_folder);

//         assert_eq!(does_contain_folder, false);
//     }

//     #[test]
//     fn rename_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let rename_folder = &Outline {
//             text: "Rename Folder".to_string(),
//             title: Some("Rename Folder".to_string()),
//             ..Outline::default()
//         };

//         let expected_folder = &Outline {
//             text: "Expected Folder".to_string(),
//             title: Some("Expected Folder".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(
//             Event::AddNewFolder(rename_folder.text.to_string()),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::RenameFolder(
//                 rename_folder.text.to_string(),
//                 expected_folder.text.to_string(),
//             ),
//             &mut model,
//         );

//         let does_contain_folder = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .contains(expected_folder);

//         assert_eq!(does_contain_folder, true);
//     }

//     #[test]
//     fn fail_rename_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let test_folder = &Outline {
//             text: "Expected Folder".to_string(),
//             title: Some("Expected Folder".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(
//             Event::AddNewFolder(test_folder.text.to_string()),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::RenameFolder(test_folder.text.to_string(), test_folder.text.to_string()),
//             &mut model,
//         );
//         let actual_error = model.notification.message;
//         let expected_error = format!(
//             "Cannot rename folder to \"{}\". It already exists.",
//             test_folder.text.to_string()
//         );

//         assert_eq!(actual_error, expected_error);
//     }
// }

// // TODO add: title, description, type, version, html_url. are these derived or manual?
// #[cfg(test)]
// mod add_subscription {
//     use super::*;
//     use crate::{CrabNews, Event, Model};
//     use crux_core::testing::AppTester;
//     use opml::Outline;

//     #[test]
//     fn add_new_subscription_to_root() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let sub_title = "New Sub Root".to_string();
//         let sub_link = "https://example.com/atom.xml".to_string();
//         let expected_sub = &Outline {
//             text: sub_title.to_string(),
//             xml_url: Some(sub_link.to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(
//             Event::AddNewSubscription(None, sub_title.to_string(), sub_link.to_string()),
//             &mut model,
//         );

//         let does_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .contains(expected_sub);

//         assert_eq!(does_contain_sub, true);
//     }

//     #[test]
//     fn add_new_subscription_to_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "New Sub Folder".to_string();
//         let sub_title = "New Sub Folder".to_string();
//         let sub_link = "https://example.com/atom.xml".to_string();
//         let expected_sub = &Outline {
//             text: sub_title.to_string(),
//             xml_url: Some(sub_link.to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 sub_title.to_string(),
//                 sub_link.to_string(),
//             ),
//             &mut model,
//         );

//         let does_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_name)
//             .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
//             .unwrap();

//         assert_eq!(does_contain_sub, true);
//     }

//     #[test]
//     fn fail_add_new_subscription_to_root() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let sub_title = "New Sub Root".to_string();
//         let sub_link = "https://example.com/atom.xml".to_string();
//         let test_subscription = &Outline {
//             text: sub_title.to_string(),
//             xml_url: Some(sub_link.to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(
//             Event::AddNewSubscription(None, sub_title.to_string(), sub_link.to_string()),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::AddNewSubscription(None, sub_title.to_string(), sub_link.to_string()),
//             &mut model,
//         );
//         let actual_error = model.notification.message;
//         let expected_error = format!(
//             "Cannot add new subscription \"{}\". You are already subscribed.",
//             test_subscription.text.to_string()
//         );

//         assert_eq!(actual_error, expected_error);
//     }

//     #[test]
//     fn fail_add_new_subscription_to_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "New Sub Folder".to_string();
//         let sub_title = "New Sub Folder".to_string();
//         let sub_link = "https://example.com/atom.xml".to_string();
//         let test_subscription = &Outline {
//             text: sub_title.to_string(),
//             xml_url: Some(sub_link.to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 sub_title.to_string(),
//                 sub_link.to_string(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 sub_title.to_string(),
//                 sub_link.to_string(),
//             ),
//             &mut model,
//         );
//         let actual_error = model.notification.message;
//         let expected_error = format!(
//             "Cannot add new subscription \"{}\". You are already subscribed.",
//             test_subscription.text.to_string()
//         );

//         assert_eq!(actual_error, expected_error);
//     }
// }

// #[cfg(test)]
// mod delete_subscription {
//     use super::*;
//     use crate::{CrabNews, Event, Model};
//     use crux_core::testing::AppTester;
//     use opml::Outline;

//     #[test]
//     fn delete_subscription_from_root() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let deleted_sub = &Outline {
//             text: "Deleted Sub Root".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(
//             Event::AddNewSubscription(
//                 None,
//                 deleted_sub.text.to_string(),
//                 deleted_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::DeleteSubscription(None, deleted_sub.text.to_string()),
//             &mut model,
//         );

//         let does_contain_sub = model.subscriptions.opml.body.outlines.contains(deleted_sub);

//         assert_eq!(does_contain_sub, false);
//     }

//     #[test]
//     fn delete_subscription_from_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Deleted Sub Folder".to_string();
//         let deleted_sub = &Outline {
//             text: "Sub Name".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 deleted_sub.text.to_string(),
//                 deleted_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::DeleteSubscription(Some(folder_name.to_string()), deleted_sub.text.to_string()),
//             &mut model,
//         );

//         let does_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_name)
//             .find_map(|folder| Some(folder.outlines.contains(deleted_sub)))
//             .unwrap();

//         assert_eq!(does_contain_sub, false);
//     }

//     #[test]
//     fn delete_subscription_from_folder_with_multi_subs() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Deleted Multi Subs".to_string();
//         let delete_sub = &Outline {
//             text: "Deleted Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };
//         let expected_sub = &Outline {
//             text: "Expected Sub".to_string(),
//             xml_url: Some("https://example.com/".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 delete_sub.text.to_string(),
//                 delete_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::DeleteSubscription(Some(folder_name.to_string()), delete_sub.text.to_string()),
//             &mut model,
//         );

//         let does_contain_deleted_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_name)
//             .find_map(|folder| Some(folder.outlines.contains(delete_sub)))
//             .unwrap();

//         let does_contain_expected_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_name)
//             .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
//             .unwrap();

//         assert_eq!(
//             (!does_contain_deleted_sub && does_contain_expected_sub),
//             true
//         );
//     }
// }

// #[cfg(test)]
// mod rename_subscription {
//     use super::*;
//     use crate::{CrabNews, Event, Model};
//     use crux_core::testing::AppTester;
//     use opml::Outline;

//     #[test]
//     fn rename_subscription_in_root() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let rename_sub = &Outline {
//             text: "Old Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };
//         let expected_sub = &Outline {
//             text: "Renamed Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(
//             Event::AddNewSubscription(
//                 None,
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::RenameSubscription(
//                 None,
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//                 expected_sub.text.to_string(),
//             ),
//             &mut model,
//         );

//         let does_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .contains(expected_sub);

//         assert_eq!(does_contain_sub, true);
//     }

//     #[test]
//     fn fail_rename_subscription_in_root() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let rename_sub = &Outline {
//             text: "Old Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(
//             Event::AddNewSubscription(
//                 None,
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::RenameSubscription(
//                 None,
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//                 rename_sub.text.to_string(),
//             ),
//             &mut model,
//         );

//         let actual_error = model.notification.message;
//         let expected_error = format!(
//             "Cannot rename subscription to \"{}\". It already exists.",
//             rename_sub.text.to_string(),
//         );

//         assert_eq!(actual_error, expected_error);
//     }

//     #[test]
//     fn rename_subscription_in_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Renamed Sub Folder".to_string();
//         let rename_sub = &Outline {
//             text: "Old Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };
//         let expected_sub = &Outline {
//             text: "Renamed Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::RenameSubscription(
//                 Some(folder_name.to_string()),
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//                 expected_sub.text.to_string(),
//             ),
//             &mut model,
//         );

//         let does_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_name)
//             .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
//             .unwrap();

//         assert_eq!(does_contain_sub, true);
//     }

//     #[test]
//     fn fail_rename_subscription_in_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Renamed Sub Folder".to_string();
//         let rename_sub = &Outline {
//             text: "Old Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::RenameSubscription(
//                 Some(folder_name.to_string()),
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//                 rename_sub.text.to_string(),
//             ),
//             &mut model,
//         );

//         let actual_error = model.notification.message;
//         let expected_error = format!(
//             "Cannot rename subscription to \"{}\". It already exists.",
//             rename_sub.text.to_string(),
//         );

//         assert_eq!(actual_error, expected_error);
//     }

//     #[test]
//     fn rename_subscription_in_folder_with_multi_subs() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Renamed Multi Sub Folder".to_string();
//         let untouched_sub = &Outline {
//             text: "Untouched Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };
//         let rename_sub = &Outline {
//             text: "Old Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };
//         let expected_sub = &Outline {
//             text: "Renamed Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 untouched_sub.text.to_string(),
//                 untouched_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::RenameSubscription(
//                 Some(folder_name.to_string()),
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//                 expected_sub.text.to_string(),
//             ),
//             &mut model,
//         );

//         let does_contain_untouched_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_name)
//             .find_map(|folder| Some(folder.outlines.contains(untouched_sub)))
//             .unwrap();

//         let does_contain_expected_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_name)
//             .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
//             .unwrap();

//         assert_eq!(
//             (does_contain_untouched_sub && does_contain_expected_sub),
//             true
//         );
//     }

//     #[test]
//     fn fail_rename_subscription_in_folder_with_multi_subs() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Renamed Multi Sub Folder".to_string();
//         let untouched_sub = &Outline {
//             text: "Untouched Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };
//         let rename_sub = &Outline {
//             text: "Old Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 untouched_sub.text.to_string(),
//                 untouched_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::RenameSubscription(
//                 Some(folder_name.to_string()),
//                 rename_sub.text.to_string(),
//                 rename_sub.xml_url.clone().unwrap().clone(),
//                 rename_sub.text.to_string(),
//             ),
//             &mut model,
//         );

//         let actual_error = model.notification.message;
//         let expected_error = format!(
//             "Cannot rename subscription to \"{}\". It already exists.",
//             rename_sub.text.to_string(),
//         );

//         assert_eq!(actual_error, expected_error);
//     }
// }

// #[cfg(test)]
// mod move_subscription {
//     use super::*;
//     use crate::{CrabNews, Event, Model};
//     use crux_core::testing::AppTester;
//     use opml::Outline;

//     #[test]
//     fn move_subscription_from_root_to_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Move Sub To Folder".to_string();
//         let expected_sub = &Outline {
//             text: "Moved Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 None,
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::MoveSubscriptionToFolder(
//                 expected_sub.clone(),
//                 None,
//                 Some(folder_name.to_string()),
//             ),
//             &mut model,
//         );

//         let does_root_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .contains(expected_sub);

//         let does_folder_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_name)
//             .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
//             .unwrap();

//         assert_eq!((!does_root_contain_sub && does_folder_contain_sub), true);
//     }

//     #[test]
//     fn fail_move_subscription_from_root_to_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Move Sub To Folder".to_string();
//         let expected_sub = &Outline {
//             text: "Moved Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 None,
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::MoveSubscriptionToFolder(
//                 expected_sub.clone(),
//                 None,
//                 Some(folder_name.to_string()),
//             ),
//             &mut model,
//         );

//         let actual_error = model.notification.message;
//         let expected_error = format!(
//             "Cannot move subscription to \"{}\". It already exists.",
//             expected_sub.text.to_string(),
//         );

//         assert_eq!(actual_error, expected_error);
//     }

//     #[test]
//     fn move_subscription_from_folder_to_root() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Move Sub To Root".to_string();
//         let expected_sub = &Outline {
//             text: "Moved Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::MoveSubscriptionToFolder(
//                 expected_sub.clone(),
//                 Some(folder_name.to_string()),
//                 None,
//             ),
//             &mut model,
//         );

//         let does_root_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .contains(expected_sub);

//         let does_folder_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_name)
//             .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
//             .unwrap();

//         assert_eq!((does_root_contain_sub && !does_folder_contain_sub), true);
//     }

//     #[test]
//     fn fail_move_subscription_from_folder_to_root() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_name = "Move Sub To Root".to_string();
//         let expected_sub = &Outline {
//             text: "Moved Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_name.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_name.to_string()),
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 None,
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::MoveSubscriptionToFolder(
//                 expected_sub.clone(),
//                 Some(folder_name.to_string()),
//                 None,
//             ),
//             &mut model,
//         );

//         let actual_error = model.notification.message;
//         let expected_error = format!(
//             "Cannot move subscription to \"{}\". It already exists.",
//             expected_sub.text.to_string(),
//         );

//         assert_eq!(actual_error, expected_error);
//     }

//     #[test]
//     fn move_subscription_from_folder_to_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_one = "Folder One".to_string();
//         let folder_two = "Folder Two".to_string();
//         let expected_sub = &Outline {
//             text: "Moved Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_one.to_string()), &mut model);
//         let _ = app.update(Event::AddNewFolder(folder_two.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_one.to_string()),
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::MoveSubscriptionToFolder(
//                 expected_sub.clone(),
//                 Some(folder_one.to_string()),
//                 Some(folder_two.to_string()),
//             ),
//             &mut model,
//         );

//         let does_folder_one_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_one)
//             .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
//             .unwrap();

//         let does_folder_two_contain_sub = model
//             .subscriptions
//             .opml
//             .body
//             .outlines
//             .iter_mut()
//             .filter(|outline| outline.text == folder_two)
//             .find_map(|folder| Some(folder.outlines.contains(expected_sub)))
//             .unwrap();

//         assert_eq!(
//             (!does_folder_one_contain_sub && does_folder_two_contain_sub),
//             true
//         );
//     }

//     #[test]
//     fn fail_move_subscription_from_folder_to_folder() {
//         let app = AppTester::<CrabNews, _>::default();
//         let mut model: Model = Model::default();
//         let folder_one = "Folder One".to_string();
//         let folder_two = "Folder Two".to_string();
//         let expected_sub = &Outline {
//             text: "Moved Sub".to_string(),
//             xml_url: Some("https://example.com/atom.xml".to_string()),
//             ..Outline::default()
//         };

//         let _ = app.update(Event::AddNewFolder(folder_one.to_string()), &mut model);
//         let _ = app.update(Event::AddNewFolder(folder_two.to_string()), &mut model);
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_one.to_string()),
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::AddNewSubscription(
//                 Some(folder_two.to_string()),
//                 expected_sub.text.to_string(),
//                 expected_sub.xml_url.clone().unwrap().clone(),
//             ),
//             &mut model,
//         );
//         let _ = app.update(
//             Event::MoveSubscriptionToFolder(
//                 expected_sub.clone(),
//                 Some(folder_one.to_string()),
//                 Some(folder_two.to_string()),
//             ),
//             &mut model,
//         );

//         let actual_error = model.notification.message;
//         let expected_error = format!(
//             "Cannot move subscription to \"{}\". It already exists.",
//             expected_sub.text.to_string(),
//         );

//         assert_eq!(actual_error, expected_error);
//     }
// }

// ANCHOR_END: tests
