use rbar::RBar;

mod bar;
mod config;
mod modules;
mod rbar;

fn main() {
    let app = RBar::new();

    // todo: handle errors
    if let Err(_) = app.start() {
        // eprintln!("{}", err);
        std::process::exit(1);
    }
}
