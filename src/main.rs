use rbar::RBar;

mod bar;
mod config;
mod error;
mod modules;
mod rbar;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");

    let app = RBar::new();

    if let Err(e) = app.start() {
        log::error!("There was an error while running app: {}", e);
        std::process::exit(1);
    }
}
