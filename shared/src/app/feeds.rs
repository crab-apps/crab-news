use feed_rs::model::Feed;
use feed_rs::parser::{parse, ParseErrorKind, ParseFeedError};
use serde::{Deserialize, Serialize};

// TODO upon a successful subscription, populate Feed from model.subscriptions and generate a Feed ID
// https://docs.rs/feed-rs/latest/feed_rs/model/struct.Feed.html
// https://docs.rs/feed-rs/latest/feed_rs/parser/fn.generate_id_from_link_and_title.html

// TODO to populate Feed use Get xml_url and callback -> https://docs.rs/feed-rs/latest/feed_rs/parser/struct.Parser.html#method.parse
// NOTE beware not computing existing ones
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Feeds {
    pub feeds: Vec<Feed>,
}

impl Feeds {
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
