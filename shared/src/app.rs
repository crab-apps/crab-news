use crux_core::{render::Render, App};
use opml::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    Increment,
    Decrement,
    Reset,
    ImportOPML, // shows up in Menu -> File
    ExportOPML, // shows up in Menu -> File
}

#[derive(Default)]
pub struct Model {
    count: isize,
    opml: OPML, // to deal with file.opml import/export
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ViewModel {
    pub count: String,
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Default)]
pub struct Counter;

impl App for Counter {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match event {
            Event::Increment => model.count += 1,
            Event::Decrement => model.count -= 1,
            Event::Reset => model.count = 0,
            Event::ImportOPML => model.count = 0,
            Event::ExportOPML => model.count = 0,
        };

        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            count: format!("Count is: {}", model.count),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crux_core::{assert_effect, testing::AppTester};

    #[test]
    fn renders() {
        let app = AppTester::<Counter, _>::default();
        let mut model = Model::default();

        let update = app.update(Event::Reset, &mut model);

        // Check update asked us to `Render`
        assert_effect!(update, Effect::Render(_));
    }

    #[test]
    fn import_opml() {
        let mut file = File::open("example.opml").unwrap();
        let document = OPML::from_reader(&mut file).unwrap();
        let example_xml = r#"<?xml version="1.0" encoding="ISO-8859-1"?><opml version="2.0"><head><title>Subscriptions.opml</title><dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated><ownerName>Crab News</ownerName></head><body><outline text="Gentle Wash Records" title="Gentle Wash Records" description="" type="rss" version="RSS" htmlUrl="https://gentlewashrecords.com/" xmlUrl="https://gentlewashrecords.com/atom.xml"/></body></opml>"#;
        let xml = OPML::from_str(example_xml).unwrap();
        assert_eq!(xml, document);
    }
}
