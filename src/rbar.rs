use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, OnceLock,
};

use gtk4::{prelude::*, Application};
use tokio::runtime::Runtime;

use crate::{
    bar::{load_bars, load_css},
    config::Config,
};

const APP_ID: &str = "com.migueldamota.rbar";

#[derive(Debug)]
pub struct RBar {
    pub config: Config,
}

impl RBar {
    pub fn new() -> Self {
        Self {
            config: Config::load(),
        }
    }

    pub fn start(self) -> Result<(), ()> {
        let app = Application::builder().application_id(APP_ID).build();

        let instance = Arc::new(self);

        app.connect_activate(move |app| {
            // Load styles.
            load_css();

            load_bars(instance.clone(), app);
        });

        // Let's run it.
        app.run();

        Ok(())
    }

    pub fn unique_id() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    pub fn runtime() -> Arc<Runtime> {
        static RUNTIME: OnceLock<Arc<Runtime>> = OnceLock::new();
        RUNTIME.get_or_init(|| Arc::new(create_runtime())).clone()
    }
}

fn create_runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create runtime")
}
