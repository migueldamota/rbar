use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, OnceLock,
    },
};

use gtk::{glib::ExitCode, prelude::*, Application};
use tokio::runtime::Runtime;

use crate::style;

use crate::{bar::load_bars, config::Config};

const APP_ID: &str = "com.migueldamota.rbar";

#[derive(Debug)]
pub struct RBar {
    pub config: Config,
    pub config_dir: PathBuf,
}

impl RBar {
    pub fn new() -> Self {
        let (config, config_dir) = Config::load();
        Self { config, config_dir }
    }

    /// Start the rbar bar.
    pub fn start(self) -> crate::Result<ExitCode> {
        let app = Application::builder().application_id(APP_ID).build();

        let instance = Arc::new(self);

        app.connect_activate(move |app| {
            // Load styles.
            style::init(instance.clone());

            // Load bars.
            load_bars(instance.clone(), app);
        });

        // Let's run it.
        Ok(app.run())
    }

    /// Get unique id for widgets.
    ///
    /// This is always increments by 1 on each call.
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
