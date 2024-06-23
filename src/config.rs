use serde::Deserialize;

/// The general configuration of the bar.
#[derive(Debug, Deserialize)]
pub struct Config {}

impl Config {
    /// Load a configuration from a file.
    pub fn load() -> Self {
        Self {}
    }
}

/// A bar configuration.
#[derive(Debug, Deserialize)]
pub struct BarConfig {}
