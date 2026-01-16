use super::subscriptions::SubscriptionTitle;
use super::Error;

use feed_rs::model::Feed;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Feeds {
    pub feeds: Vec<Feed>,
}

trait AddFeed {
    fn add_feed(&self, body: Vec<u8>) -> Result<Self, Error>
    where
        Self: Sized;
}

impl AddFeed for Feeds {
    fn add_feed(&self, body: Vec<u8>) -> Result<Self, Error> {
        let mut feeds = self.clone();
        let feed = feed_rs::parser::parse(&*body)?;
        feeds.feeds.push(feed);
        Ok(feeds)
    }
}

trait FindFeed {
    fn find_feed(&self, feed_title: &SubscriptionTitle) -> Result<Feed, Error>;
}

impl FindFeed for Feeds {
    fn find_feed(&self, feed_title: &SubscriptionTitle) -> Result<Feed, Error> {
        let no_feed_found = Error::set_error(
            "Cannot find feed",
            feed_title.as_ref(),
            "for the specified subscription.",
        );

        let feed = self
            .feeds
            .iter()
            .find(|feed| feed.title.clone().unwrap().content == feed_title.to_string());

        if let Some(feed) = feed {
            Ok(feed.clone())
        } else {
            Err(no_feed_found)
        }
    }
}

impl Feeds {
    pub(super) fn add_feed(&self, body: Vec<u8>) -> Result<Self, Error> {
        <Self as AddFeed>::add_feed(self, body)
    }

    pub(super) fn find_feed(&self, feed_title: &SubscriptionTitle) -> Result<Feed, Error> {
        <Self as FindFeed>::find_feed(self, feed_title)
    }
}

// #[cfg(test)]
// mod feeds {
// use super::*;
// use crate::{Account, AccountType, Accounts, AccountsExt};
// use crate::{CrabNews, Effect, Event, Model};

// mode shell START
// use anyhow::Result;
// use crux_core::Core;
// use crux_http::protocol::{HttpRequest, HttpResponse, HttpResult};
// use std::collections::VecDeque;

// #[allow(clippy::large_enum_variant)]
// enum Task {
//     Event(Event),
//     Effect(Effect),
// }

// pub(crate) fn run(core: &Core<CrabNews>, event: Event) -> Result<Vec<HttpRequest>> {
//     let mut queue: VecDeque<Task> = VecDeque::new();

//     queue.push_back(Task::Event(event));

//     let mut received: Vec<HttpRequest> = vec![];

//     while !queue.is_empty() {
//         let task = queue.pop_front().expect("an event");

//         match task {
//             Task::Event(event) => {
//                 enqueue_effects(&mut queue, core.process_event(event));
//             }
//             Task::Effect(effect) => match effect {
//                 Effect::Render(_) => (),
//                 Effect::Http(mut request) => {
//                     let http_request = &request.operation;

//                     received.push(http_request.clone());
//                     let response = HttpResponse::ok().json("Hello").build();

//                     enqueue_effects(
//                         &mut queue,
//                         core.resolve(&mut request, HttpResult::Ok(response))
//                             .expect("effect should resolve"),
//                     );
//                 }
//             },
//         };
//     }

//     Ok(received)
// }

// fn enqueue_effects(queue: &mut VecDeque<Task>, effects: Vec<Effect>) {
//     queue.append(&mut effects.into_iter().map(Task::Effect).collect())
// }
// mod shell END

// #[test]
// fn get_feed() -> Result<(), Box<dyn std::error::Error>> {
//     let app = CrabNews;
//     let mut model: Model = Model::default();
//     let sub_title = "Gentle Wash Records".to_string();
//     let sub_link = "https://gentlewashrecords.com/atom.xml".to_string();

//     let account = Account::new(&AccountType::Local);
//     let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
//     let account_index = Accounts::find_account_index(&model.accounts, &account);

//     let _ = app.update(
//         Event::AddSubscription(account.clone(), None, sub_title, sub_link.to_string()),
//         &mut model, &()
//     );

//     let core: Core<CrabNews> = Core::default();

//     let received = run(&core, Event::GetFeed(account, sub_link.to_string()))?;

//     assert_eq!(received, vec![HttpRequest::get(sub_link).build()]);
//     Ok(())
// }

// #[test]
// fn add_new_feed() {
//     let app = CrabNews;
//     let mut model: Model = Model::default();

//     let account = Account::new(&AccountType::Local);
//     let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
//     let account_index = Accounts::find_account_index(&model.accounts, &account);

//     let sub_title = "Gentle Wash Records".to_string();
//     let example_rss = r#"<?xml version="1.0" encoding="UTF-8" ?>
//       <rss version="2.0">
//         <channel>
//           <title>Gentle Wash Records</title>
//           <description>This is an example of an RSS feed</description>
//           <link>http://www.example.com/main.html</link>
//           <lastBuildDate>Mon, 06 Sep 2010 00:01:00 +0000</lastBuildDate>
//           <pubDate>Sun, 06 Sep 2009 16:20:00 +0000</pubDate>
//           <ttl>1800</ttl>

//           <item>
//             <title>Example entry</title>
//             <description>Here is some text containing an interesting description.</description>
//             <link>http://www.example.com/blog/post/1</link>
//             <guid isPermaLink="true">7bd204c6-1655-4c27-aeee-53f933c5395f</guid>
//             <pubDate>Sun, 06 Sep 2009 16:20:00 +0000</pubDate>
//           </item>

//         </channel>
//       </rss>"#;

//     let body = Vec::from(example_rss.as_bytes());
//     let _ = app.update(Event::SetFeed(account, example_rss.as_bytes()), &mut model, &());
//     let added_feed = Subscriptions::find_feed(&model.accounts.acct[account_index].subs, &sub_title);

//     assert_eq!(added_feed.title.unwrap().content, sub_title);
// }

// #[test]
// fn fail_add_feed_already_exists() {
//     let app = CrabNews;
//     let mut model: Model = Model::default();
//     let account = Account::new(&AccountType::Local);
//     let account_index = Accounts::find_account_index(&model.accounts, &account);
//     let sub_title = "Gentle Wash Records".to_string();
//     let sub_link = "https://gentlewashrecords.com/atom.xml".to_string();

//     let _ = app.update(Event::CreateAccount(AccountType::Local), &mut model, &());
//     let _ = app.update(
//         Event::AddNewSubscription(
//             account.clone(),
//             None,
//             sub_title.to_string(),
//             sub_link.to_string(),
//         ),
//         &mut model, &()
//     );
//     let _response = app
//         .update(Event::GetFeed(account.clone(), sub_link), &mut model, &())
//         .expect_one_event();
//     // let _ = app.update(Event::SetFeed(account.clone(), response), &mut model, &());

//     let added_feed = Subscriptions::find_feed(&model.accounts.acct[account_index].subs, &sub_title);
//     let added_feed_title = added_feed.title.clone().unwrap().content;
//     let actual_error = model.notification.message;
//     let expected_error = format!(
//         "Cannot add new subscription \"{}\". You are already subscribed.",
//         added_feed_title
//     );

//     assert_eq!(actual_error, expected_error);
// }
// }
