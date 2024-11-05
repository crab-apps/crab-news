// ANCHOR: app
use crux_core::{render::Render, App};
use feed_rs::model::Feed;
use serde::{Deserialize, Serialize};
// use crux_http::Http;
// use url::Url;

// NOTE - crate: https://crates.io/crates/opml
// - to deal with subscriptions and outlines:
mod subscriptions;
pub use subscriptions::{
    FolderName, NewFolder, NewName, OldFolder, OldLink, OldName, OpmlFile, OpmlName, Subscription,
    SubscriptionLink, SubscriptionTitle, Subscriptions,
};

// ANCHOR: events
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    // EVENTS FROM THE SHELL
    ImportSubscriptions(OpmlFile),
    ExportSubscriptions(OpmlName),
    AddNewFolder(FolderName),
    DeleteFolder(FolderName),
    RenameFolder(OldName, NewName),
    AddNewSubscription(Option<FolderName>, SubscriptionTitle, SubscriptionLink),
    DeleteSubscription(Option<FolderName>, SubscriptionTitle),
    RenameSubscription(Option<FolderName>, OldName, OldLink, NewName),
    MoveSubscriptionToFolder(Subscription, OldFolder, NewFolder),

    // EVENTS LOCAL TO THE CORE
    #[serde(skip)]
    Fetch(crux_http::Result<crux_http::Response<Feed>>),
}
// ANCHOR_END: events

// ANCHOR: model
#[derive(Default, Serialize)]
pub struct Model {
    notification: Notification,
    subscriptions: Subscriptions,
    // TODO populate these at some poing. With feed-rs?
    subscription_folder: FolderName,
    subscription_title: SubscriptionTitle,
    subscription_link: SubscriptionLink,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Notification {
    pub title: String,
    pub message: String,
}

// ANCHOR: view model
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ViewModel {
    pub notification: Notification,
    pub subscriptions: Subscriptions,
    // TODO populate these at some poing. With feed-rs?
    pub subscription_folder: FolderName,
    pub subscription_title: SubscriptionTitle,
    pub subscription_link: SubscriptionLink,
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
                match Subscriptions::import(&model.subscriptions, subs_opml_file) {
                    // TODO on duplicates, prompt user for merge or replace
                    Ok(subs) => model.subscriptions = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Import Error".to_string(),
                            message: err.to_string(),
                        }
                    }
                };
                ()
            }
            Event::ExportSubscriptions(subs_opml_name) => {
                match Subscriptions::export(&model.subscriptions, subs_opml_name) {
                    Ok(success) => {
                        return model.notification = Notification {
                            title: "Subscriptions Exported".to_string(),
                            message: success.to_string(),
                        }
                    }
                    Err(err) => {
                        // TODO once shell is implemented, check failures
                        return model.notification = Notification {
                            title: "Export Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                };
            }
            Event::AddNewFolder(folder_name) => {
                match Subscriptions::add_folder(&model.subscriptions, folder_name) {
                    Ok(subs) => model.subscriptions = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "New Folder Error".to_string(),
                            message: err.to_string(),
                        }
                    }
                };
                ()
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
                        }
                    }
                };
                ()
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
                        }
                    }
                };
                ()
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
                        }
                    }
                };
                ()
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
                        }
                    }
                };
                ()
            }
            Event::Fetch(_) => todo!(),
        };

        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            notification: model.notification.clone(),
            subscriptions: model.subscriptions.clone(),
            subscription_folder: model.subscription_folder.to_string(),
            subscription_title: model.subscription_title.to_string(),
            subscription_link: model.subscription_link.to_string(),
            // feeds: model.feeds.clone(),
        }
    }
}
// ANCHOR_END: impl_app
// ANCHOR_END: app

// ANCHOR: tests
// ANCHOR_END: tests
