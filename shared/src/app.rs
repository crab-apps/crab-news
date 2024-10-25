use crux_core::{render::Render, App};
// use crux_http::Http;
use feed_rs::{model::Feed, parser};
use opml::{Head, Outline, OPML};
use serde::{Deserialize, Serialize};
use std::fs::File;
// use url::Url;
use chrono::prelude::Local;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    ImportSubscriptions(String),
    ExportSubscriptions(String, String),
    AddNewSubscription,
    DeleteSubscription,
    RenameSubscription,
    MoveSubscriptionToFolder,
    AddNewFolder,
    DeleteFolder,
    RenameFolder,

    // EVENTS LOCAL TO THE CORE
    #[serde(skip)]
    Fetch(crux_http::Result<crux_http::Response<Feed>>),
}

#[derive(Default)]
pub struct Model {
    subscriptions: OPML,
    // subscription_folder: Outline,
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
            Event::ImportSubscriptions(subs_opml_file) => {
                let mut file = File::open(subs_opml_file).unwrap();
                model.subscriptions = OPML::from_reader(&mut file).unwrap();
            }
            Event::ExportSubscriptions(subs_opml_file, subs_opml_name) => {
                let xml_tag = r#"<?xml version="1.0" encoding="ISO-8859-1"?>"#.to_string();
                let custom_head = Head {
                    title: Some(subs_opml_name),
                    date_created: Some(Local::now().format("%Y, %a %b %e %T").to_string()),
                    owner_name: Some("Crab News".to_string()),
                    ..Head::default()
                };
                let custon_opml = OPML {
                    head: Some(custom_head),
                    body: model.subscriptions.body.clone(),
                    ..OPML::default()
                };
                let export_content = xml_tag + &custon_opml.to_string().unwrap();
                let _ = std::fs::write(subs_opml_file, &export_content);
            }
            Event::AddNewSubscription => todo!(),
            Event::DeleteSubscription => todo!(),
            Event::RenameSubscription => todo!(),
            Event::MoveSubscriptionToFolder => todo!(),
            Event::AddNewFolder => todo!(),
            Event::DeleteFolder => todo!(),
            Event::RenameFolder => todo!(),
            Event::Fetch(_) => todo!(),
        };

        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {}
    }
}

// FIXME change these to test Events/Capabilities
// for now I'm understanding crates by laying down
// "batch jobs" to get familiar with "the flow of things"
#[cfg(test)]
mod test {
    use super::*;
    use crux_core::testing::AppTester;

    #[test]
    fn import_subscriptions() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let subs_opml_file = "example_import.opml".to_string();

        let _ = app.update(Event::ImportSubscriptions(subs_opml_file), &mut model);
        let added_subs = model.subscriptions;

        let example_opml = r#"<opml version="2.0"><head><title>Subscriptions.opml</title><dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated><ownerName>Crab News</ownerName></head><body><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/><outline text="Group Name" title="Group Name"><outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/></outline></body></opml>"#;
        let expected_subs = OPML::from_str(example_opml).unwrap();

        assert_eq!(added_subs, expected_subs);
    }

    #[test]
    fn export_subscriptions() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model: Model = Model::default();
        let subs_opml_file = "example_export.opml".to_string();
        let subs_opml_name = "Subscriptions.opml".to_string();
        let date_created = Some(Local::now().format("%Y, %a %b %e %T").to_string());

        let example_subs = format!("<opml version=\"2.0\"><head><title>Subscriptions.opml</title><dateCreated>{}</dateCreated><ownerName>Crab News</ownerName></head><body><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/atom.xml\"/><outline text=\"Group Name\" title=\"Group Name\"><outline text=\"Feed Name\" title=\"Feed Name\" description=\"\" type=\"rss\" version=\"RSS\" htmlUrl=\"https://example.com/\" xmlUrl=\"https://example.com/rss.xml\"/></outline></body></opml>", date_created.unwrap());
        model.subscriptions = OPML::from_str(&example_subs).unwrap();
        let import_content = model.subscriptions.clone();

        let _ = app.update(
            Event::ExportSubscriptions(subs_opml_file.clone(), subs_opml_name.clone()),
            &mut model,
        );

        let mut exported_file = std::fs::File::open(subs_opml_file.clone()).unwrap();
        let export_content = OPML::from_reader(&mut exported_file).unwrap();

        assert_eq!(export_content, import_content);
    }

    // TODO use Events::AddNewSubscription(FeedStore::Root)
    // https://docs.rs/opml/1.1.6/opml/struct.OPML.html#method.add_feed
    #[test]
    fn add_new_subscription_to_root() {
        let mut model: Model = Model::default();
        let mut new_sub: OPML = OPML::default();

        new_sub.add_feed("Feed Name", "https://example.com/");
        model.subscriptions = new_sub;

        let added_feed = model.subscriptions.body.outlines.first().unwrap();

        let expected_feed = &Outline {
            text: "Feed Name".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        assert_eq!(added_feed, expected_feed);
    }

    // TODO use Events::AddNewSubscription(FeedStore::Folder)
    // https://docs.rs/opml/1.1.6/opml/struct.Outline.html#method.add_feed
    #[test]
    fn add_new_subscription_to_folder() {
        let mut model = Model::default();
        let new_sub = OPML::default();
        let new_folder = Outline {
            text: "Folder Name".to_string(),
            title: Some("Folder Name".to_string()),
            ..Outline::default()
        };

        model.subscriptions = new_sub;
        let mut body = model.subscriptions.body.clone();
        body.outlines.push(new_folder.clone());
        let mut first_outline = body.outlines.first().unwrap().clone();
        first_outline.add_feed("Feed Name", "https://example.com/");

        let added_feed = first_outline.outlines.first().unwrap();
        let expected_feed = &Outline {
            text: "Feed Name".to_string(),
            xml_url: Some("https://example.com/".to_string()),
            ..Outline::default()
        };

        assert_eq!(added_feed, expected_feed);
    }

    // TODO use Events::DeleteSubscription
    #[test]
    fn delete_subscription() {}

    // TODO use Events::RenameSubscription
    #[test]
    fn rename_subscription() {}

    // TODO use Events::AddNewFolder
    // THIS ADDS AN OUTLINE TO OPML::Body
    #[test]
    fn add_new_folder() {
        let mut model: Model = Model::default();
        let new_sub: OPML = OPML::default();
        model.subscriptions = new_sub;
        let mut body: opml::Body = model.subscriptions.body.clone();
        let new_folder = &Outline {
            text: "Folder Name".to_string(),
            title: Some("Folder Name".to_string()),
            ..Outline::default()
        };

        body.outlines.push(new_folder.clone());

        let added_folder = body.outlines.first().unwrap();
        let expected_folder = new_folder;

        assert_eq!(added_folder, expected_folder);
    }

    // TODO use Events::DeleteFolder
    #[test]
    fn delete_folder() {}

    // TODO use Events::RenameFolder
    #[test]
    fn rename_folder() {}
}
