use rbar::RBar;
use tracing::{debug, error};

mod bar;
mod config;
mod error;
mod modules;
mod rbar;
mod style;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");

    use tracing::Level;
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    debug!(
        "Using GSK_RENDERER: {}",
        std::env::var("GSK_RENDERER").unwrap()
    );

    let app = RBar::new();

    if let Err(e) = app.start() {
        error!("There was an error while running app: {}", e);
        std::process::exit(1);
    }
}
