use crux_core::{
    macros::effect,
    render::{render, RenderOperation},
    App, Command,
};
use crux_http::{command::Http, protocol::HttpRequest};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod error;
pub use error::Error;

mod settings;
pub use settings::*;

mod accounts;
pub use accounts::*;

mod subscriptions;
pub use subscriptions::*;

// ANCHOR: events
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
#[non_exhaustive]
pub enum Event {
    // EVENTS FROM THE SHELL
    GetPreferences,
    // SetPreferences(Preferences),
    CreateAccount(AccountType),
    DeleteAccount(Account),
    RenameAccount(OldAccountName, NewAccountName),
    ImportSubscriptions(Account, OpmlFile),
    ExportSubscriptions(Account, OpmlName),
    AddNewFolder(Account, FolderName),
    DeleteFolder(Account, FolderName),
    RenameFolder(Account, OldFolderName, NewFolderName),
    AddSubscription(
        Account,
        Option<FolderName>,
        SubscriptionTitle,
        SubscriptionLink,
    ),
    DeleteSubscription(Account, Option<FolderName>, SubscriptionTitle),
    RenameSubscription(
        Account,
        Option<FolderName>,
        OldFolderName,
        OldLink,
        NewFolderName,
    ),
    MoveSubscription(Account, Subscription, OldFolder, NewFolder),
    GetFeed(Account, SubscriptionLink),

    // EVENTS LOCAL TO THE CORE
    #[serde(skip)]
    SetFeed(Account, crux_http::Result<crux_http::Response<Vec<u8>>>),
}
// ANCHOR_END: events

// ANCHOR: effects and capabilities
#[effect(typegen)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Effect {
    Render(RenderOperation),
    Http(HttpRequest),
}
// ANCHOR_END: effects and capabilities

// ANCHOR: model
#[derive(Default, Serialize)]
#[non_exhaustive]
pub struct Model {
    ////////////////////////////
    // preferences UI
    pub preferences: HashMap<String, String>,
    pub content_body_text_size: ContentBodyTextSize,
    pub browser: Browser,
    pub opening_method: OpeningMethod,
    pub refresh_interval: RefreshInterval,
    ////////////////////////////
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
// ANCHOR_END: model

// ANCHOR: view model
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[non_exhaustive]
pub struct ViewModel {
    pub notification: Notification,
    pub account_name: String,    // extrapolated from account
    pub folder_name: FolderName, // root or folder if None -> nothing? root? phantom?
    pub subscription_name: SubscriptionTitle, // extrapolated from feed
                                 // pub accounts: Accounts,
}
// ANCHOR_END: view model

// ANCHOR: app
#[derive(Default)]
pub struct CrabNews;

impl App for CrabNews {
    type Model = Model;
    type Event = Event;
    type ViewModel = ViewModel;
    type Capabilities = (); // will be deprecated, so use unit type for now
    type Effect = Effect;

    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
        _caps: &(), // will be deprecated, so prefix with underscore for now
    ) -> Command<Effect, Event> {
        match event {
            Event::GetPreferences => match settings::read_config() {
                Ok(preferences) => {
                    model.preferences = preferences;
                    render()
                }
                Err(err) => {
                    model.notification = Notification {
                        title: "Preferences Error".to_string(),
                        message: err.to_string(),
                    };
                    render()
                }
            },
            Event::CreateAccount(account_type) => {
                match Accounts::add_account(&model.accounts, &account_type) {
                    Ok(accts) => {
                        model.accounts = accts;
                        render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Account Error".to_string(),
                            message: err.to_string(),
                        };
                        render()
                    }
                }
            }
            Event::DeleteAccount(account) => {
                model.accounts = Accounts::delete_account(&model.accounts, &account);
                render()
            }
            Event::RenameAccount(old_account_name, new_account_name) => {
                match Accounts::rename_account(
                    &model.accounts,
                    &old_account_name,
                    &new_account_name,
                ) {
                    Ok(accts) => {
                        model.accounts = accts;
                        render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Account Error".to_string(),
                            message: err.to_string(),
                        };
                        render()
                    }
                }
            }
            Event::ImportSubscriptions(account, subs_opml_file) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::import(
                    &model.accounts.acct[account_index].subs,
                    &subs_opml_file,
                ) {
                    // TODO on duplicates, prompt user for merge or replace
                    Ok(subs) => {
                        model.accounts.acct[account_index].subs = subs;
                        render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Import Error".to_string(),
                            message: err.to_string(),
                        };
                        render()
                    }
                }
            }
            Event::ExportSubscriptions(account, subs_opml_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::export(
                    &model.accounts.acct[account_index].subs,
                    &subs_opml_name,
                ) {
                    Ok(success) => {
                        model.notification = Notification {
                            title: "Subscriptions Exported".to_string(),
                            message: success.to_string(),
                        };
                        render()
                    }
                    Err(err) => {
                        // TODO once shell is implemented, check failures
                        model.notification = Notification {
                            title: "Export Error".to_string(),
                            message: err.to_string(),
                        };
                        render()
                    }
                }
            }
            Event::AddNewFolder(account, folder_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::add_folder(
                    &model.accounts.acct[account_index].subs,
                    &folder_name,
                ) {
                    Ok(subs) => {
                        model.accounts.acct[account_index].subs = subs;
                        render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "New Folder Error".to_string(),
                            message: err.to_string(),
                        };
                        render()
                    }
                }
            }
            Event::DeleteFolder(account, folder_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                model.accounts.acct[account_index].subs = {
                    Subscriptions::delete_folder(
                        &model.accounts.acct[account_index].subs,
                        &folder_name,
                    )
                };
                render()
            }
            Event::RenameFolder(account, old_folder_name, new_folder_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::rename_folder(
                    &model.accounts.acct[account_index].subs,
                    &old_folder_name,
                    &new_folder_name,
                ) {
                    Ok(subs) => {
                        model.accounts.acct[account_index].subs = subs;
                        render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Rename Folder Error".to_string(),
                            message: err.to_string(),
                        };
                        render()
                    }
                }
            }
            Event::AddSubscription(account, folder_name, sub_title, sub_link) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::add_subscription(
                    &model.accounts.acct[account_index].subs,
                    &folder_name,
                    &sub_title,
                    &sub_link,
                ) {
                    Ok(subs) => {
                        model.accounts.acct[account_index].subs = subs;
                        render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                        render()
                    }
                }
                // caps.http
                //     .get(sub_link)
                //     .send(move |result| Event::SetFeed(account, result));
            }
            Event::DeleteSubscription(account, folder_name, sub_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                model.accounts.acct[account_index].subs = Subscriptions::delete_subscription(
                    &model.accounts.acct[account_index].subs,
                    &folder_name,
                    &sub_name,
                );
                render()
            }
            Event::RenameSubscription(account, folder_name, old_title, old_link, new_name) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::rename_subscription(
                    &model.accounts.acct[account_index].subs,
                    &folder_name,
                    &old_title,
                    &old_link,
                    &new_name,
                ) {
                    Ok(subs) => {
                        model.accounts.acct[account_index].subs = subs;
                        render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                        render()
                    }
                }
            }
            Event::MoveSubscription(account, subscription, old_folder, new_folder) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                match Subscriptions::move_subscription(
                    &model.accounts.acct[account_index].subs,
                    &subscription,
                    &old_folder,
                    &new_folder,
                ) {
                    Ok(subs) => {
                        model.accounts.acct[account_index].subs = subs;
                        render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Subscription Error".to_string(),
                            message: err.to_string(),
                        };
                        render()
                    }
                }
            }
            Event::GetFeed(account, sub_link) => Http::get(&sub_link)
                .build()
                .then_send(move |result| Event::SetFeed(account, result)),
            Event::SetFeed(account, Ok(mut response)) => {
                let account_index = Accounts::find_account_index(&model.accounts, &account);
                let body = response.take_body().unwrap();
                match Subscriptions::add_feed(&model.accounts.acct[account_index].subs, body) {
                    Ok(subs) => {
                        model.accounts.acct[account_index].subs = subs;
                        render()
                    }
                    Err(err) => {
                        model.notification = Notification {
                            title: "Feed Error".to_string(),
                            message: err.to_string(),
                        };
                        render()
                    }
                }
            }
            Event::SetFeed(_, Err(err)) => {
                model.notification = Notification {
                    title: "Http Error".to_string(),
                    message: err.to_string(),
                };
                render()
            }
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            notification: model.notification.clone(),
            account_name: model.account_name.clone(),
            folder_name: model.folder_name.clone(), // root or folder if None -> nothing? root? phantom?
            subscription_name: model.subscription_name.clone(), // extrapolated from feed
                                                    // accounts: model.accounts.clone(),
                                                    // feeds: model.accounts.
                                                    // subscriptions: model.subscriptions.clone(),
                                                    // subscription_folder: model.subscription_folder.to_string(),
                                                    // subscription_title: model.subscription_title.to_string(),
                                                    // subscription_link: model.subscription_link.to_string(),
        }
    }
}
// ANCHOR_END: app

// ANCHOR: test
// ANCHOR_END: tests
