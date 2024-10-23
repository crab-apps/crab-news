use crux_core::{render::Render, App};
// use crux_http::Http;
use feed_rs::{model::Feed, parser};
use opml::{Outline, OPML};
use serde::{Deserialize, Serialize};
// use std::fs::File;
// use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    ImportSubscriptions,
    ExportSubscriptions,
    // Feeds
    // TO ADD AN OUTLINE TO ROOT USE https://docs.rs/opml/1.1.6/opml/struct.OPML.html#method.add_feed
    // TO ADD AN OUTLINE TO FOLDER USE https://docs.rs/opml/1.1.6/opml/struct.Outline.html#method.add_feed
    AddNewFeed, // account | root | folder // shows up in Menu -> File
    DeleteFeed,
    RenameFeed,
    MoveFeedToFolder, // location -> root | folder
    // FeedStore -> root + 1st level folder. no more
    // THIS ADDS AN OUTLINE TO OPML::Body
    AddNewFolder, // shows up in Menu -> File
    DeleteFolder,
    RenameFolder,

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
            Event::MoveFeedToFolder => todo!(),
            Event::Fetch(_) => todo!(),
            Event::AddNewFolder => todo!(),
            Event::DeleteFolder => todo!(),
            Event::RenameFolder => todo!(),
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

    // TODO Events::ImportSubscriptions,
    #[test]
    fn import_subscriptions() {
        let model: Model = Model::default();
        let mut subscriptions: Vec<OPML> = model.subscriptions;
        let mut file = std::fs::File::open("example_import.opml").unwrap();
        let document = OPML::from_reader(&mut file).unwrap();
        let example_feed = r#"<?xml version="1.0" encoding="ISO-8859-1"?><opml version="2.0"><head><title>Subscriptions.opml</title><dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated><ownerName>Crab News</ownerName></head><body><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/><outline text="Group Name" title="Group Name"><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/></outline></body></opml>"#;

        subscriptions.push(document);

        let added_feed = subscriptions.first().unwrap().clone();
        let expected_feed = OPML::from_str(example_feed).unwrap();

        assert_eq!(added_feed, expected_feed);
    }

    // TODO Events::ExportSubscriptions,
    #[test]
    fn export_subscriptions() {
        let model: Model = Model::default();
        let mut subscriptions: Vec<OPML> = model.subscriptions;
        let example_feed = r#"<?xml version="1.0" encoding="ISO-8859-1"?><opml version="2.0"><head><title>Subscriptions.opml</title><dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated><ownerName>Crab News</ownerName></head><body><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/><outline text="Group Name" title="Group Name"><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/></outline></body></opml>"#;
        let current_subscriptions = OPML::from_str(example_feed).unwrap();
        subscriptions.push(current_subscriptions);

        let exported_feed = subscriptions.first().unwrap().to_string().unwrap();
        let _ = std::fs::write("example_export.opml", &exported_feed);

        let mut file = std::fs::File::open("example_export.opml").unwrap();
        let document = OPML::from_reader(&mut file).unwrap();
        let expected_feed = document.to_string().unwrap();

        assert_eq!(exported_feed, expected_feed);
    }

    // TODO use Events::AddNewFeed(FeedStore::Root)
    #[test]
    fn add_new_root_feed() {
        let model: Model = Model::default();
        let mut subscriptions: Vec<OPML> = model.subscriptions;
        let mut new_sub: OPML = OPML::default();

        new_sub.add_feed("Feed Name", "https://example.com/");
        subscriptions.push(new_sub);

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

    // TODO use Events::AddNewFeed(FeedStore::Folder)
    #[test]
    fn add_new_nested_feed() {
        // let model: Model = Model::default();
        // let subscriptions: Vec<OPML> = model.subscriptions;
        let mut group = Outline::default();

        // <outline text="Group Name" title="Group Name">
        group.add_feed("Feed Name", "https://example.com/");

        let added_feed = group.outlines.first().unwrap();
        let expected_feed = &Outline {
            text: "Feed Name".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        assert_eq!(added_feed, expected_feed);
    }

    // TODO use Events::AddNewFolder
    #[test]
    fn add_new_folder() {
        let new_sub: OPML = OPML::default();
        let mut model: Model = Model::default();
        model.subscriptions.push(new_sub);
        let mut body: opml::Body = model.subscriptions.first().unwrap().body.clone();
        let new_folder = &Outline {
            text: "Folder Name".to_string(),
            title: Some("Folder Name".to_string()),
            ..Outline::default()
        };

        body.outlines.push(new_folder.clone());

        let added_folder = body.outlines.first().unwrap();
        let expected_folder = &Outline {
            text: "Folder Name".to_string(),
            title: Some("Folder Name".to_string()),
            ..Outline::default()
        };

        assert_eq!(added_folder, expected_folder);
    }
}
