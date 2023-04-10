use serde::Deserialize;
use std::io::Read;
#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    pub open_file: String,
    pub save_file: String,
}

impl Config {
    pub fn config_from_file() -> Option<Self> {
        let Ok(home) = std::env::var("HOME") else {
            return None
        };
        let config_path = std::path::Path::new(home.as_str())
            .join(".config")
            .join("xdg-desktop-portal-shana")
            .join("config.toml");
        let Ok(mut file) = std::fs::OpenOptions::new()
            .read(true)
            .open(config_path)
        else {
            return None
        };
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_err() {
            return None;
        };
        toml::from_str(&buf).unwrap_or(None)
    }
}

impl From<String> for super::PortalSelect {
    fn from(value: String) -> Self {
        if value == "Gnome" {
            Self::Gnome
        } else {
            Self::Kde
        }
    }
}

impl From<Option<Config>> for super::ProtalConfig {
    fn from(value: Option<Config>) -> Self {
        match value {
            None => crate::ProtalConfig {
                savefile: crate::PortalSelect::Gnome,
                openfile: crate::PortalSelect::Gnome,
            },
            Some(value) => crate::ProtalConfig {
                savefile: value.save_file.into(),
                openfile: value.open_file.into(),
            },
        }
    }
}