use std::{fs, path::PathBuf};

use crate::battery::Battery;

pub struct Manager {
    root: PathBuf,
}

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

impl Manager {
    pub fn new() -> Self {
        Self {
            root: PathBuf::from("/sys/class/power_supply"),
        }
    }

    /// Get all batteries.
    pub fn batteries(&self) -> Result<Vec<Battery>> {
        Ok(fs::read_dir(&self.root)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .starts_with("BAT")
            })
            .filter_map(|p| read_battery(p).ok())
            .collect::<Vec<_>>())
    }
}

fn read_battery(path: PathBuf) -> Result<Battery> {
    println!("Reading battery at {:?}", path);
    Ok(Battery::with_root(path))
}
