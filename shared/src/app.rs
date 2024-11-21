// ANCHOR: app
// ANCHOR: imports
use crux_core::{render::Render, App};
use crux_http::Http;
// use crux_platform::{Platform, PlatformResponse};
use serde::{Deserialize, Serialize};
// use url::Url;

mod accounts;
pub use accounts::{AccountType, Accounts};

// NOTE - crate: https://crates.io/crates/opml
// to deal with subscriptions and outlines:
mod subscriptions;
pub use subscriptions::{
    FolderName, NewFolder, NewName, OldFolder, OldLink, OldName, OpmlFile, OpmlName, Subscription,
    SubscriptionLink, SubscriptionTitle, Subscriptions,
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
    ImportSubscriptions(OpmlFile),
    ExportSubscriptions(OpmlName),
    AddNewFolder(FolderName),
    DeleteFolder(FolderName),
    RenameFolder(OldName, NewName),
    AddNewSubscription(Option<FolderName>, SubscriptionTitle, SubscriptionLink),
    DeleteSubscription(Option<FolderName>, SubscriptionTitle),
    RenameSubscription(Option<FolderName>, OldName, OldLink, NewName),
    MoveSubscriptionToFolder(Subscription, OldFolder, NewFolder),
    // Get,
    // EVENTS LOCAL TO THE CORE
    // #[serde(skip)]
    // Set(crux_http::Result<crux_http::Response<Feed>>),
}
// ANCHOR_END: events

// ANCHOR: model
#[derive(Default, Serialize)]
pub struct Model {
    notification: Notification,
    accounts: Accounts, // TODO move subscriptions into accounts and write new tests
    subscriptions: Subscriptions, // FIXME this is temporary to make old tests pass
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Notification {
    pub title: String,
    pub message: String,
}

// ANCHOR: view model
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
                match Accounts::add_account(&model.accounts, &account_type) {
                    Ok(accts) => model.accounts = accts,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Account Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::DeleteAccount(account_type) => {
                model.accounts = Accounts::delete_account(&model.accounts, &account_type);
            }
            Event::ImportSubscriptions(subs_opml_file) => {
                match Subscriptions::import(&model.subscriptions, subs_opml_file) {
                    // TODO on duplicates, prompt user for merge or replace
                    Ok(subs) => model.subscriptions = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Import Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::ExportSubscriptions(subs_opml_name) => {
                match Subscriptions::export(&model.subscriptions, subs_opml_name) {
                    Ok(success) => {
                        return model.notification = Notification {
                            title: "Subscriptions Exported".to_string(),
                            message: success.to_string(),
                        };
                    }
                    Err(err) => {
                        // TODO once shell is implemented, check failures
                        return model.notification = Notification {
                            title: "Export Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::AddNewFolder(folder_name) => {
                match Subscriptions::add_folder(&model.subscriptions, folder_name) {
                    Ok(subs) => model.subscriptions = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "New Folder Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::DeleteFolder(folder_name) => {
                model.subscriptions =
                    Subscriptions::delete_folder(&model.subscriptions, folder_name);
            }
            Event::RenameFolder(old_folder_name, new_folder_name) => {
                match Subscriptions::rename_folder(
                    &model.subscriptions,
                    old_folder_name,
                    new_folder_name,
                ) {
                    Ok(subs) => model.subscriptions = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Rename Folder Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::AddNewSubscription(folder_name, sub_title, sub_link) => {
                match Subscriptions::add_subscription(
                    &model.subscriptions,
                    folder_name,
                    sub_title,
                    sub_link,
                ) {
                    Ok(subs) => model.subscriptions = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::DeleteSubscription(folder_name, sub_name) => {
                model.subscriptions =
                    Subscriptions::delete_subscription(&model.subscriptions, folder_name, sub_name);
            }
            Event::RenameSubscription(folder_name, old_title, old_link, new_name) => {
                match Subscriptions::rename_subscription(
                    &model.subscriptions,
                    folder_name,
                    old_title,
                    old_link,
                    new_name,
                ) {
                    Ok(subs) => model.subscriptions = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::MoveSubscriptionToFolder(subscription, old_folder, new_folder) => {
                match Subscriptions::move_subscription(
                    &model.subscriptions,
                    subscription,
                    old_folder,
                    new_folder,
                ) {
                    Ok(subs) => model.subscriptions = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            } // Event::Get => {
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
// ANCHOR_END: tests
