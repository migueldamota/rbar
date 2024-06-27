use rbar::RBar;

mod bar;
mod config;
mod error;
mod modules;
mod rbar;

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");

    let app = RBar::new();

    // todo: handle errors
    if app.start().is_err() {
        // eprintln!("{}", err);
        std::process::exit(1);
    }
}
