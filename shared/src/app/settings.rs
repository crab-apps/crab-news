use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const PREFS_FILE: &str = "/Users/andreacfromtheapp/.config/crab-news/preferences.toml";

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Preferences {
    pub theme_mode: ThemeMode,
    pub light_theme: LightTheme,
    pub dark_theme: DarkTheme,
    pub text_size: TextSize,
    pub browser: Browser,
    pub opening_method: OpeningMethod,
    pub refresh_interval: RefreshInterval,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    Auto,
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum LightTheme {
    #[default]
    Light,
    Cupcake,
    Bumblebee,
    Emerald,
    Corporate,
    Retro,
    Cyberpunk,
    Valentine,
    Garden,
    Lofi,
    Pastel,
    Fantasy,
    Wireframe,
    Cmyk,
    Autumn,
    Acid,
    Lemonade,
    Winter,
    Nord,
    Caramellatte,
    Silk,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum DarkTheme {
    #[default]
    Dark,
    Synthwave,
    Halloween,
    Forest,
    Aqua,
    Black,
    Luxury,
    Dracula,
    Business,
    Night,
    Coffee,
    Dim,
    Sunset,
    Abyss,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum TextSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
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

pub fn read_config() -> Result<HashMap<String, String>, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::from_str(PREFS_FILE, config::FileFormat::Toml))
        .build()?;

    Ok(settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap())
}

#[cfg(test)]
mod configurations {
    use super::*;
    use crate::{CrabNews, Event, Model};
    use crux_core::App;

    #[test]
    fn config_read() {
        let app = CrabNews;
        let mut model = Model::default();
        let mut expected_config = HashMap::new();

        expected_config.insert("test".to_string(), "\"dada\"".to_string());

        let _ = app.update(Event::GetPreferences, &mut model, &());

        assert_eq!(expected_config, model.preferences)
    }
}
