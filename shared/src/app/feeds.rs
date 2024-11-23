use super::SubscriptionTitle;
use feed_rs::model::Feed;
// use feed_rs::parser::{parse, ParseErrorKind, ParseFeedError};
use serde::{Deserialize, Serialize};

// TODO upon a successful subscription, populate Feed from model.subscriptions
// https://docs.rs/feed-rs/latest/feed_rs/model/struct.Feed.html

// TODO to populate Feed use Get xml_url and callback -> https://docs.rs/feed-rs/latest/feed_rs/parser/struct.Parser.html#method.parse
// NOTE beware not computing existing ones
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Feeds {
    pub feeds: Vec<Feed>,
}

impl Feeds {
    // ANCHOR: helper functions
    pub fn find_feed_index(&self, sub_title: &SubscriptionTitle) -> usize {
        self.feeds
            .iter()
            .position(|f| f.title.clone().unwrap().content == *sub_title)
            .unwrap()
    }
    // ANCHOR_END: helper functions

    // pub fn get_feed(&self, xml_url: String) -> Result<Feed, ParseFeedError> {
    // match parse(xml_url) {
    // Ok(f) => f,
    // Err(e) => return Err(e),
    // }
    // }
}

// #[cfg(test)]
// mod tests {
// use super::*;
// use crate::{CrabNews, Event, Model};
// use crux_core::testing::AppTester;

// #[test]
// fn get_feed() {}
// }
