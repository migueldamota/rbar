use std::path::PathBuf;

use serde::Deserialize;
use std::fs;

use crate::modules::Modules;

/// The general configuration of the bar.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub bar: BarConfig,
    pub margin: MarginConfig,
}

impl Config {
    /// Load a configuration from a file.
    pub fn load() -> (Self, PathBuf) {
        // todo: handle errors
        let config_dir = Config::get_path();

        let file = fs::File::open(&config_dir).expect("File to exist");
        (
            serde_json::from_reader(file).expect("Config to be created"),
            config_dir,
        )
    }

    pub fn get_dir() -> PathBuf {
        dirs::config_dir().map_or_else(|| PathBuf::from("."), |dir| dir.join("rbar"))
    }

    /// Get path to config file.
    ///
    /// Default: `~/.config/rbar/config.json`
    pub fn get_path() -> PathBuf {
        Self::get_dir().join("config.json")
    }

    pub fn get_style_path() -> PathBuf {
        Self::get_dir().join("style.css")
    }
}

/// A bar configuration.
#[derive(Debug, Default, Deserialize)]
pub struct BarConfig {
    pub height: i32,
    pub modules: Vec<Modules>,
}

/// Margin configuration.
#[derive(Debug, Default, Deserialize)]
pub struct MarginConfig {
    #[serde(default = "margin_default")]
    pub top: i32,
    #[serde(default = "margin_default")]
    pub left: i32,
    #[serde(default = "margin_default")]
    pub right: i32,
    #[serde(default = "margin_default")]
    pub bottom: i32,
}

fn margin_default() -> i32 {
    0
}
