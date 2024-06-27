use serde::Deserialize;

/// The general configuration of the bar.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub bar: BarConfig,
    pub margin: MarginConfig,
}

impl Config {
    /// Load a configuration from a file.
    pub fn load() -> Self {
        Self {
            margin: MarginConfig {
                left: 8,
                right: 8,
                top: 8,
                bottom: 8,
            },
            bar: BarConfig { height: 40 },
        }
    }
}

/// A bar configuration.
#[derive(Debug, Deserialize)]
pub struct BarConfig {
    pub height: i32,
}

/// Margin configuration
#[derive(Debug, Deserialize)]
pub struct MarginConfig {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
}
