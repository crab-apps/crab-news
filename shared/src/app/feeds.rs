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
// mod feeds { }
