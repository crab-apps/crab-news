// ANCHOR: app
// ANCHOR: imports
use crux_core::{render::Render, App};
use crux_http::Http;
use feed_rs::model::Feed;
use serde::{Deserialize, Serialize};

mod accounts;
pub use accounts::{Account, AccountType, Accounts};

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
#[derive(Serialize, Deserialize)]
pub enum Event {
    // EVENTS FROM THE SHELL
    CreateAccount(AccountType),
    DeleteAccount(Account),
    ImportSubscriptions(Account, OpmlFile),
    ExportSubscriptions(Account, OpmlName),
    AddNewFolder(Account, FolderName),
    DeleteFolder(Account, FolderName),
    RenameFolder(Account, OldName, NewName),
    AddNewSubscription(
        Account,
        Option<FolderName>,
        SubscriptionTitle,
        SubscriptionLink,
    ),
    DeleteSubscription(Account, Option<FolderName>, SubscriptionTitle),
    RenameSubscription(Account, Option<FolderName>, OldName, OldLink, NewName),
    MoveSubscriptionToFolder(Account, Subscription, OldFolder, NewFolder),
    GetFeed(String),

    // EVENTS LOCAL TO THE CORE
    #[serde(skip)]
    SetFeed(crux_http::Result<crux_http::Response<Vec<u8>>>),
}
// ANCHOR_END: events

// ANCHOR: model
#[derive(Default, Serialize)]
pub struct Model {
    notification: Notification,
    accounts: Accounts,
    feeds: Vec<Feed>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
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

// ANCHOR: impl_app
impl App for CrabNews {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match event {
            Event::CreateAccount(account_type) => {
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
            Event::DeleteAccount(account) => {
                model.accounts = Accounts::delete_account(&model.accounts, &account);
            }
            Event::ImportSubscriptions(account, subs_opml_file) => {
                let acct_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::import(&model.accounts.accts[acct_index].subs, &subs_opml_file)
                {
                    // TODO on duplicates, prompt user for merge or replace
                    Ok(subs) => model.accounts.accts[acct_index].subs = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Import Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::ExportSubscriptions(account, subs_opml_name) => {
                let acct_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::export(&model.accounts.accts[acct_index].subs, &subs_opml_name)
                {
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
            Event::AddNewFolder(account, folder_name) => {
                let acct_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::add_folder(
                    &model.accounts.accts[acct_index].subs,
                    &folder_name,
                ) {
                    Ok(subs) => model.accounts.accts[acct_index].subs = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "New Folder Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::DeleteFolder(account, folder_name) => {
                let acct_index = Accounts::find_account_index(&model.accounts, &account);
                model.accounts.accts[acct_index].subs = Subscriptions::delete_folder(
                    &model.accounts.accts[acct_index].subs,
                    &folder_name,
                );
            }
            Event::RenameFolder(account, old_folder_name, new_folder_name) => {
                let acct_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::rename_folder(
                    &model.accounts.accts[acct_index].subs,
                    &old_folder_name,
                    &new_folder_name,
                ) {
                    Ok(subs) => model.accounts.accts[acct_index].subs = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Rename Folder Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::AddNewSubscription(account, folder_name, sub_title, sub_link) => {
                let acct_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::add_subscription(
                    &model.accounts.accts[acct_index].subs,
                    &folder_name,
                    &sub_title,
                    &sub_link,
                ) {
                    Ok(subs) => model.accounts.accts[acct_index].subs = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
                // caps.http.get(sub_link).send(Event::SetFeed);
            }
            Event::DeleteSubscription(account, folder_name, sub_name) => {
                let acct_index = Accounts::find_account_index(&model.accounts, &account);
                model.accounts.accts[acct_index].subs = Subscriptions::delete_subscription(
                    &model.accounts.accts[acct_index].subs,
                    &folder_name,
                    &sub_name,
                );
            }
            Event::RenameSubscription(account, folder_name, old_title, old_link, new_name) => {
                let acct_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::rename_subscription(
                    &model.accounts.accts[acct_index].subs,
                    &folder_name,
                    &old_title,
                    &old_link,
                    &new_name,
                ) {
                    Ok(subs) => model.accounts.accts[acct_index].subs = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::MoveSubscriptionToFolder(account, subscription, old_folder, new_folder) => {
                let acct_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::move_subscription(
                    &model.accounts.accts[acct_index].subs,
                    &subscription,
                    &old_folder,
                    &new_folder,
                ) {
                    Ok(subs) => model.accounts.accts[acct_index].subs = subs,
                    Err(err) => {
                        return model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                    }
                }
            }
            Event::GetFeed(sub_link) => {
                caps.http.get(sub_link).send(Event::SetFeed);
            }
            Event::SetFeed(Ok(mut response)) => {
                let body = response.take_body().unwrap();
                let feed = feed_rs::parser::parse(&*body).unwrap();
                model.feeds.push(feed);
            }
            Event::SetFeed(Err(err)) => {
                return model.notification = Notification {
                    title: "Feed Error".to_string(),
                    message: err.to_string(),
                };
            }
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
