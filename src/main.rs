use rbar::RBar;

mod bar;
mod config;
mod modules;
mod rbar;

#[tokio::main]
async fn main() {
    let app = RBar::new();

    // todo: handle errors
    if app.start().is_err() {
        // eprintln!("{}", err);
        std::process::exit(1);
    }
}
