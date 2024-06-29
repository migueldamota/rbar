use std::{path::PathBuf, sync::Arc};

use gtk::{ffi::GTK_STYLE_PROVIDER_PRIORITY_USER, gdk::Display, gio::File, CssProvider};
use tracing::{debug, warn};

use crate::{config::Config, rbar::RBar};

pub fn init(rbar: Arc<RBar>) {
    let path = Config::get_style_path();
    if path.exists() {
        load_css(path);
    } else {
        warn!(
            "Style file does not exist: {} - Using default styling.",
            path.display()
        );
    }
}

fn load_css(path: PathBuf) {
    let provider = CssProvider::new();

    provider.load_from_file(&File::for_path(&path));
    debug!("Loaded css from '{}'", path.display());

    let screen = match Display::default() {
        Some(display) => display,
        None => {
            warn!("Failed to get default display");
            return;
        }
    };

    gtk::style_context_add_provider_for_display(
        &screen,
        &provider,
        GTK_STYLE_PROVIDER_PRIORITY_USER as u32,
    );
}
