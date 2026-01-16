// THIS IS COMPLETELY WRONG. CAN'T HAVE LOGIC IN THE APP.
// MAKE A PORT (OR USE CRUX_KV???) TO THE DRIVEN ADAPTER/REPOSITORY
// TO STORE THE PREFERENCES IN A DATABASE
// use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Preferences {
    pub content_body_text_size: ContentBodyTextSize,
    pub browser: Browser,
    pub opening_method: OpeningMethod,
    pub refresh_interval: RefreshInterval,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum ContentBodyTextSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Browser {
    #[default]
    Default,
    Brave,
    Chrome,
    DuckDuckGo,
    Edge,
    Firefox,
    LibreWolf,
    Mullvad,
    Opera,
    Safari,
    Tor,
    Ungoogled,
    Vivaldi,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum OpeningMethod {
    #[default]
    Background,
    Foreground,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum RefreshInterval {
    #[default]
    MinutesFifteen,
    MinutesThirty,
    HoursOne,
    HoursTwo,
    HoursThree,
    HoursFour,
}

// pub fn read_config() -> Result<HashMap<String, String>, ConfigError> {
//     let settings = Config::builder()
//         .add_source(config::File::from_str("wrong", config::FileFormat::Toml))
//         .build()?;

//     Ok(settings
//         .try_deserialize::<HashMap<String, String>>()
//         .unwrap())
// }

// #[cfg(test)]
// mod configurations {
//     use super::*;
//     use crate::{CrabNews, Event, Model};
//     use crux_core::App;

//     #[test]
//     fn config_read() {
//         let app = CrabNews;
//         let mut model = Model::default();
//         let mut expected_config = HashMap::new();

//         expected_config.insert("test".to_string(), "\"dada\"".to_string());

//         let _ = app.update(Event::GetPreferences, &mut model, &());

//         assert_eq!(expected_config, model.preferences)
//     }
// }
