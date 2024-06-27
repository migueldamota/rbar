use glib::Propagation;
use gtk::{
    ffi::GTK_STYLE_PROVIDER_PRIORITY_USER,
    gdk::{Display, Monitor, Screen},
    gio,
    prelude::*,
    Application, ApplicationWindow, CssProvider, Orientation, StyleContext, WindowType,
};
use gtk_layer_shell::LayerShell;

use std::{env, sync::Arc};

use crate::{
    modules::{clock::Clock, BarModuleFactory, Modules},
    rbar::RBar,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Bar {
    name: &'static str,

    window: ApplicationWindow,
    content: gtk::Box,
    left: gtk::Box,
    center: gtk::Box,
    right: gtk::Box,

    rbar: Arc<RBar>,
}

impl Bar {
    pub fn new(app: &Application, rbar: Arc<RBar>) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .type_(WindowType::Toplevel)
            .build();

        let name = "rbar";

        window.style_context().add_class("bar");

        let content = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(false)
            .height_request(rbar.config.bar.height)
            .name("bar")
            .build();

        content.style_context().add_class("content");

        let left = create_container("start");
        let center = create_container("center");
        let right = create_container("end");

        content.add(&left);
        content.set_center_widget(Some(&center));
        content.pack_end(&right, false, false, 0);

        window.add(&content);

        window.connect_destroy_event(|_, _| {
            gtk::main_quit();
            Propagation::Proceed
        });

        Self {
            name,
            window,
            content,
            rbar,

            left,
            center,
            right,
        }
    }

    pub fn init(self, monitor: &Monitor) -> Result<Self> {
        println!(
            "Initializing bar '{}' on {:?}",
            self.name,
            monitor.manufacturer()
        );

        self.setup_layer_shell(&self.window, monitor);

        let _res = self.load_modules();

        self.show();

        Ok(self)
    }

    fn setup_layer_shell(&self, win: &ApplicationWindow, monitor: &Monitor) {
        use gtk_layer_shell::{Edge, Layer};

        win.init_layer_shell();
        win.set_monitor(monitor);
        win.set_layer(Layer::Top);
        win.set_namespace(env!("CARGO_PKG_NAME"));

        win.auto_exclusive_zone_enable();

        let margin = &self.rbar.config.margin;
        win.set_layer_shell_margin(Edge::Top, margin.top);
        win.set_layer_shell_margin(Edge::Left, margin.left);
        win.set_layer_shell_margin(Edge::Right, margin.right);
        win.set_layer_shell_margin(Edge::Bottom, margin.bottom);

        win.set_anchor(Edge::Top, true);
        win.set_anchor(Edge::Left, true);
        win.set_anchor(Edge::Right, true);
    }

    fn show(&self) {
        self.content.show_all();

        self.window.show();
    }

    fn load_modules(&self) -> Result<()> {
        add_modules(&self.center, &self.rbar)?;

        Ok(())
    }
}

pub fn load_css() {
    let style_path = env::current_dir().expect("to exist").join("style.css");

    let provider = CssProvider::new();
    if let Err(err) = provider.load_from_file(&gio::File::for_path(&style_path)) {
        eprintln!("Failed to load CSS: {}", err);
    } else {
        println!("CSS loaded from: {}", style_path.display());
    }

    let screen = Screen::default().expect("Failed to get defautl GTK screen");
    StyleContext::add_provider_for_screen(
        &screen,
        &provider,
        GTK_STYLE_PROVIDER_PRIORITY_USER as u32,
    );
}

fn create_container(name: &str) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .name(name)
        .build();

    container.style_context().add_class("container");

    container
}

fn get_display() -> Display {
    use std::process::exit;

    Display::default().map_or_else(|| exit(3), |display| display)
}

pub fn load_bars(rbar: Arc<RBar>, app: &Application) {
    let display = get_display();

    for i in 0..display.n_monitors() {
        let monitor = display.monitor(i).expect("monitor to exist");
        let _ = create_bar(app, rbar.clone(), &monitor);
    }
}

fn create_bar(app: &Application, rbar: Arc<RBar>, monitor: &Monitor) -> Result<Bar> {
    let bar = Bar::new(app, rbar);
    bar.init(monitor)
}

fn add_modules(content: &gtk::Box, rbar: &Arc<RBar>) -> Result<()> {
    let factory = BarModuleFactory::new(rbar.clone());

    let modules: Vec<Modules> = vec![Modules::Clock(Box::new(Clock::new()))];

    for module in modules {
        module.create(&factory, content)?;
    }

    Ok(())
}
