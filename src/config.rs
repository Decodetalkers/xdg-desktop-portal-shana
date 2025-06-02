use serde::Deserialize;
use std::io::Read;
#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    pub open_file: String,
    pub save_file: String,
    pub save_files: Option<String>,
    pub tips: Option<Tips>,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Tips {
    pub open_file_when_folder: String,
}

impl Config {
    pub fn config_from_file() -> Option<Self> {
        let Ok(home) = std::env::var("HOME") else {
            return None;
        };
        let config_path = std::path::Path::new(home.as_str())
            .join(".config")
            .join("xdg-desktop-portal-shana")
            .join("config.toml");
        let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(config_path) else {
            return None;
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
        match value.as_str() {
            "Gnome" => Self::Gnome,
            "Kde" => Self::Kde,
            "Lxqt" => Self::Lxqt,
            "Gtk" => Self::Gtk,
            value => Self::Other(value.to_string()),
        }
    }
}

impl From<Option<Config>> for super::ProtalConfig {
    fn from(value: Option<Config>) -> Self {
        match value {
            None => crate::ProtalConfig {
                savefile: crate::PortalSelect::Gtk,
                openfile: crate::PortalSelect::Gtk,
                savefiles: crate::PortalSelect::Gtk,
                openfile_casefolder: crate::PortalSelect::Gtk,
            },
            Some(value) => crate::ProtalConfig {
                savefile: value.save_file.into(),
                openfile: value.open_file.clone().into(),
                savefiles: value.save_files.unwrap_or("Gtk".to_string()).into(),
                openfile_casefolder: match value.tips {
                    None => value.open_file.into(),
                    Some(v) => v.open_file_when_folder.into(),
                },
            },
        }
    }
}

#[test]
fn tst_toml() {
    let config_src1 = include_str!("../misc/test/config1.toml");
    let config1: super::ProtalConfig = Some(toml::from_str(config_src1).unwrap()).into();
    assert_eq!(
        config1,
        super::ProtalConfig {
            openfile: crate::PortalSelect::Kde,
            savefile: crate::PortalSelect::Gnome,
            savefiles: crate::PortalSelect::Gtk,
            openfile_casefolder: crate::PortalSelect::Lxqt,
        }
    );
    let config_src2 = include_str!("../misc/test/config2.toml");
    let config2: super::ProtalConfig = Some(toml::from_str(config_src2).unwrap()).into();
    assert_eq!(
        config2,
        super::ProtalConfig {
            openfile: crate::PortalSelect::Kde,
            savefile: crate::PortalSelect::Gnome,
            savefiles: crate::PortalSelect::Gtk,
            openfile_casefolder: crate::PortalSelect::Kde,
        }
    );
}
