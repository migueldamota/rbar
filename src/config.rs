use std::path::PathBuf;

use serde::Deserialize;
use std::fs;

use crate::modules::Modules;

/// The general configuration of the bar.
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub bar: BarConfig,
    pub margin: MarginConfig,
}

impl Config {
    /// Load a configuration from a file.
    pub fn load() -> Self {
        // todo: handle errors
        let file = fs::File::open(Config::get_path()).expect("File to exist");
        serde_json::from_reader(file).expect("Config to be created")
    }

    /// Get path to config file.
    ///
    /// Default: `~/.config/rbar/config.json`
    fn get_path() -> PathBuf {
        let mut path = dirs::config_dir().expect("to exist");
        path.push("rbar");
        path.push("config.json");

        path
    }
}

/// A bar configuration.
#[derive(Debug, Deserialize)]
pub struct BarConfig {
    pub height: i32,
    pub modules: Vec<Modules>,
}

/// Margin configuration
#[derive(Debug, Deserialize)]
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
