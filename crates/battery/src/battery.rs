use std::{fs, path::PathBuf};

type Result<T, E = Box<dyn std::error::Error + Send>> = std::result::Result<T, E>;

#[derive(Clone, Default, Debug)]
pub struct Battery {
    pub root: PathBuf,

    state: State,
    soc: f32,
}

impl Battery {
    pub fn with_root(root: PathBuf) -> Self {
        let mut battery = Self {
            root,
            ..Default::default()
        };

        battery.read().unwrap();

        battery
    }

    /// Get current battery state.
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Check if battery is full.
    pub fn is_full(&self) -> bool {
        self.state == State::Full || self.soc >= 100.0
    }

    /// Check if battery is charging.
    pub fn is_charging(&self) -> bool {
        self.state == State::Charging
    }

    /// Get battery state of charge.
    pub fn state_of_charge(&self) -> f32 {
        self.soc
    }

    /// Refresh battery data
    pub fn refresh(&mut self) -> Result<&Self> {
        self.read()
    }

    fn read(&mut self) -> Result<&Self> {
        self.soc = read_file(&self.root.join("capacity"))
            .unwrap()
            .parse()
            .unwrap_or(100.0);

        Ok(self)
    }
}

fn read_file(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    fs::read_to_string(path).map_err(|e| e.into())
}

/// Battery state.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum State {
    Charging,
    Discharging,
    Full,
    #[default]
    Unknown,
}
