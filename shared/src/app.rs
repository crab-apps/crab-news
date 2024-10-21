use crux_core::{render::Render, App};
// use crux_http::Http;
use feed_rs::{model::Feed, parser};
use opml::{Outline, OPML};
use serde::{Deserialize, Serialize};
// use std::fs::File;
// use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    ImportSubscriptions, // shows up in Menu -> File
    ExportSubscriptions, // shows up in Menu -> File
    AddNewFeed,          // account | root | folder // shows up in Menu -> File
    DeleteFeed,
    RenameFeed,

    // EVENTS LOCAL TO THE CORE
    #[serde(skip)]
    Fetch(crux_http::Result<crux_http::Response<Feed>>),
}

#[derive(Default)]
pub struct Model {
    subscriptions: Vec<OPML>,
    feeds: Vec<Feed>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ViewModel {}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Default)]
pub struct CrabNews;

impl App for CrabNews {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match event {
            Event::ImportSubscriptions => todo!(),
            Event::ExportSubscriptions => todo!(),
            Event::AddNewFeed => todo!(),
            Event::DeleteFeed => todo!(),
            Event::RenameFeed => todo!(),
            Event::Fetch(_) => todo!(),
        };

        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {}
    }
}

// TODO FIXME change these to test Events/Capaabilities -- for now I'm understanding crates
#[cfg(test)]
mod test {
    use super::*;
    // use crux_core::{assert_effect, testing::AppTester};

    // #[test]
    // fn renders() {
    //     let app = AppTester::<CrabNews, _>::default();
    //     let mut model = Model::default();

    //     let update = app.update(Event::Reset, &mut model);

    //     // Check update asked us to `Render`
    //     assert_effect!(update, Effect::Render(_));
    // }

    // SUBSCRIPTIONS with opml crate
    #[test]
    // FIXME rather import opml directly into Vec<OPML> with all the right FeedStore locations folders
    fn import_subscriptions() {
        let mut file = std::fs::File::open("example.opml").unwrap();
        let document = OPML::from_reader(&mut file).unwrap();
        let example_xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?><opml version="2.0"><head><title>Subscriptions.opml</title><dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated><ownerName>Crab News</ownerName></head><body><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/><outline text="Group Name" title="Group Name"><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/></outline></body></opml>"#;
        let xml = OPML::from_str(example_xml).unwrap();
        assert_eq!(xml, document);
    }

    #[test]
    fn export_subscriptions() {}

    #[test]
    fn add_new_subscription() {
        let model: Model = Model::default();
        let mut subscriptions: Vec<OPML> = model.subscriptions;
        let mut new_sub: OPML = OPML::default();

        new_sub.add_feed("Feed Name", "https://example.com/");
        subscriptions.insert(0, new_sub);

        let added_feed = subscriptions
            .first()
            .unwrap()
            .body
            .outlines
            .first()
            .unwrap();

        let expected_feed = &Outline {
            text: "Feed Name".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        assert_eq!(added_feed, expected_feed);
    }
}
