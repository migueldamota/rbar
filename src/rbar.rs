use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use gtk::{prelude::*, Application};

use crate::{
    bar::{load_bars, load_css},
    config::Config,
};

const APP_ID: &str = "com.migueldamota.rbar";

#[derive(Debug)]
pub struct RBar {
    config: Config,
}

impl RBar {
    pub fn new() -> Self {
        Self { config: Config {} }
    }

    pub fn start(self) -> Result<(), ()> {
        let app = Application::builder().application_id(APP_ID).build();

        let instance = Arc::new(self);

        app.connect_activate(move |app| {
            load_css();

            load_bars(instance.clone(), &app);
        });

        // Let's run it.
        app.run();

        Ok(())
    }

    pub fn unique_id() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}
