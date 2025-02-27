// ANCHOR: app
// ANCHOR: imports
use crux_core::{
    render::{self, Render},
    App, Command,
};
// use crux_http::command::Http;
use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

mod accounts;
pub use accounts::{Account, AccountType, Accounts, AccountsExt};

mod subscriptions;
pub use subscriptions::{
    Feeds, FolderName, NewFolder, NewName, OldFolder, OldLink, OldName, OpmlFile, OpmlName,
    Subscription, SubscriptionLink, SubscriptionTitle, Subscriptions,
};
// ANCHOR_END: imports

// my custom Error for accounts and subscriptions
#[derive(Debug, Error)]
pub enum Error {
    #[error("{action} \"{item}\". {reason}")]
    AlreadyExists {
        action: String,
        item: String,
        reason: String,
    },
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Opml(#[from] opml::Error),
}

// ANCHOR: events
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    // EVENTS FROM THE SHELL
    CreateAccount(AccountType),
    DeleteAccount(Account),
    ImportSubscriptions(Account, OpmlFile),
    ExportSubscriptions(Account, OpmlName),
    AddNewFolder(Account, FolderName),
    DeleteFolder(Account, FolderName),
    RenameFolder(Account, OldName, NewName),
    AddSubscription(
        Account,
        Option<FolderName>,
        SubscriptionTitle,
        SubscriptionLink,
    ),
    DeleteSubscription(Account, Option<FolderName>, SubscriptionTitle),
    RenameSubscription(Account, Option<FolderName>, OldName, OldLink, NewName),
    MoveSubscription(Account, Subscription, OldFolder, NewFolder),
    // GetFeed(Account, SubscriptionLink),

    // EVENTS LOCAL TO THE CORE
    // #[serde(skip)]
    // SetFeed(Account, crux_http::Result<crux_http::Response<Vec<u8>>>),
}
// ANCHOR_END: events

// ANCHOR: model
#[derive(Default, Serialize)]
pub struct Model {
    pub notification: Notification,
    // NOTE Accounts contains Subscriptions
    // NOTE Subscriptions contains Feeds and OPML
    pub accounts: Accounts,
    pub account_name: String,                 // extrapolated from account
    pub folder_name: FolderName,              // root or folder if None -> nothing? root? phantom?
    pub subscription_name: SubscriptionTitle, // extrapolated from feed
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub title: String,
    pub message: String,
}

// ANCHOR: view model
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewModel {
    pub notification: Notification,
    pub account_name: String,                 // extrapolated from account
    pub folder_name: FolderName,              // root or folder if None -> nothing? root? phantom?
    pub subscription_name: SubscriptionTitle, // extrapolated from feed
    pub accounts: Accounts,
}
// ANCHOR_END: view model
// ANCHOR_END: model

// ANCHOR: capabilities
#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    pub render: Render<Event>,
    pub http: crux_http::Http<Event>,
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
    type Effect = Effect;

    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
        _caps: &Self::Capabilities,
    ) -> Command<Effect, Event> {
        // NOTE:
        // we no longer use the capabilities directly, but they are passed in
        // until the migration to managed effects with `Command` is complete
        // (at which point the capabilities will be removed from the `update`
        // signature). Until then we delegate to our own `update` method so that
        // we can test the app without needing to use AppTester.

        self.update(event, model)
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            notification: model.notification.clone(),
            // account_name: model.accounts.clone(),
            // folder_name: FolderName, // root or folder if None -> nothing? root? phantom?
            // subscription_name: SubscriptionTitle, // extrapolated from feed
            // accounts: model.accounts.clone(),
            // feeds: model.accounts.
            // subscriptions: model.subscriptions.clone(),
            // subscription_folder: model.subscription_folder.to_string(),
            // subscription_title: model.subscription_title.to_string(),
            // subscription_link: model.subscription_link.to_string(),
        }
    }
}

impl CrabNews {
    // NOTE:
    // this function can be moved into the `App` trait implementation, above,
    // once the `App` trait has been updated (as the final part of the migration
    // to managed effects with `Command`).
    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::CreateAccount(account_type) => {
                match Accounts::add_account(&model.accounts, &account_type) {
                    Ok(accts) => {
                        model.accounts = accts;
                        render::render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Account Error".to_string(),
                            message: err.to_string(),
                        };
                        render::render()
                    }
                }
            }
            Event::DeleteAccount(account) => {
                model.accounts = Accounts::delete_account(&model.accounts, &account);
                render::render()
            }
            Event::ImportSubscriptions(account, subs_opml_file) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::import(&model.accounts[account_index].subs, &subs_opml_file) {
                    // TODO on duplicates, prompt user for merge or replace
                    Ok(subs) => {
                        model.accounts[account_index].subs = subs;
                        render::render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Import Error".to_string(),
                            message: err.to_string(),
                        };
                        render::render()
                    }
                }
            }
            Event::ExportSubscriptions(account, subs_opml_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::export(&model.accounts[account_index].subs, &subs_opml_name) {
                    Ok(success) => {
                        model.notification = Notification {
                            title: "Subscriptions Exported".to_string(),
                            message: success.to_string(),
                        };
                        render::render()
                    }
                    Err(err) => {
                        // TODO once shell is implemented, check failures
                        model.notification = Notification {
                            title: "Export Error".to_string(),
                            message: err.to_string(),
                        };
                        render::render()
                    }
                }
            }
            Event::AddNewFolder(account, folder_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::add_folder(&model.accounts[account_index].subs, &folder_name) {
                    Ok(subs) => {
                        model.accounts[account_index].subs = subs;
                        render::render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "New Folder Error".to_string(),
                            message: err.to_string(),
                        };
                        render::render()
                    }
                }
            }
            Event::DeleteFolder(account, folder_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                model.accounts[account_index].subs = {
                    Subscriptions::delete_folder(&model.accounts[account_index].subs, &folder_name)
                };
                render::render()
            }
            Event::RenameFolder(account, old_folder_name, new_folder_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::rename_folder(
                    &model.accounts[account_index].subs,
                    &old_folder_name,
                    &new_folder_name,
                ) {
                    Ok(subs) => {
                        model.accounts[account_index].subs = subs;
                        render::render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Rename Folder Error".to_string(),
                            message: err.to_string(),
                        };
                        render::render()
                    }
                }
            }
            Event::AddSubscription(account, folder_name, sub_title, sub_link) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::add_subscription(
                    &model.accounts[account_index].subs,
                    &folder_name,
                    &sub_title,
                    &sub_link,
                ) {
                    Ok(subs) => {
                        model.accounts[account_index].subs = subs;
                        render::render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                        render::render()
                    }
                }
                // caps.http
                //     .get(sub_link)
                //     .send(move |result| Event::SetFeed(account, result));
            }
            Event::DeleteSubscription(account, folder_name, sub_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                model.accounts[account_index].subs = Subscriptions::delete_subscription(
                    &model.accounts[account_index].subs,
                    &folder_name,
                    &sub_name,
                );
                render::render()
            }
            Event::RenameSubscription(account, folder_name, old_title, old_link, new_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::rename_subscription(
                    &model.accounts[account_index].subs,
                    &folder_name,
                    &old_title,
                    &old_link,
                    &new_name,
                ) {
                    Ok(subs) => {
                        model.accounts[account_index].subs = subs;
                        render::render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                        render::render()
                    }
                }
            }
            Event::MoveSubscription(account, subscription, old_folder, new_folder) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::move_subscription(
                    &model.accounts[account_index].subs,
                    &subscription,
                    &old_folder,
                    &new_folder,
                ) {
                    Ok(subs) => {
                        model.accounts[account_index].subs = subs;
                        render::render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                        render::render()
                    }
                }
            } // Event::GetFeed(account, sub_link) => Http::get(sub_link)
              //     .build()
              //     .then_send(move |result| Event::SetFeed(account, result)),
              // Event::SetFeed(account, Ok(mut response)) => {
              //     let account_index = Accounts::find_account_index(&model.accounts, &account);
              //     let body = response.take_body().unwrap();
              //     match Subscriptions::add_feed(&model.accounts[account_index].subs, body) {
              //         Ok(subs) => {
              //             model.accounts[account_index].subs = subs;
              //             render::render()
              //         }
              //         Err(err) => {
              //             model.notification = Notification {
              //                 title: "Feed Error".to_string(),
              //                 message: err.to_string(),
              //             };
              //             render::render()
              //         }
              //     }
              // } // Event::SetFeed(_, Err(err)) => {
              //     model.notification = Notification {
              //         title: "Http Error".to_string(),
              //         message: err.to_string(),
              //     };
              //     render::render()
              // }
        }
    }
}

// ANCHOR_END: impl_app
// ANCHOR_END: app

// ANCHOR: test
// ANCHOR_END: tests
