use std::{path::PathBuf, sync::Arc};

use log::warn;

use crate::{config::Config, rbar::RBar};

pub fn init(rbar: Arc<RBar>) {
    let path = Config::get_style_path();
    if path.exists() {
        load_css(path);
    } else {
        warn!("Style file does not exist: {}", path.display());
    }
}

fn load_css(path: PathBuf) {}
