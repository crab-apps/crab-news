use feed_rs::model::Feed;
// use feed_rs::parser;
// use serde::{Deserialize, Serialize};

// TODO upon a successful subscription, populate Feed from model.subscriptions and generate a Feed ID
// https://docs.rs/feed-rs/latest/feed_rs/model/struct.Feed.html
// https://docs.rs/feed-rs/latest/feed_rs/parser/fn.generate_id_from_link_and_title.html

// TODO to populate Feed use Get xml_url and callback -> https://docs.rs/feed-rs/latest/feed_rs/parser/struct.Parser.html#method.parse
// NOTE beware not computing existing ones

// FIXME feed-rs has NO Serialize Deserialize Eq
// https://github.com/feed-rs/feed-rs/issues/164
// #[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Feeds {
    pub feeds: Vec<Feed>,
}

impl Feeds {
    pub fn get() {}
    pub fn refresh() {}
    pub fn parse() {}
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_name() {}
// }
