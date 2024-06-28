use gtk::{
    ffi::GTK_STYLE_PROVIDER_PRIORITY_USER,
    gdk::{Display, Monitor},
    gio,
    prelude::*,
    Application, ApplicationWindow, CssProvider, Orientation,
};
use gtk4_layer_shell::LayerShell;

use std::{env, sync::Arc};

use crate::{modules::ModuleFactory, rbar::RBar};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Bar {
    name: &'static str,

    window: ApplicationWindow,

    pub left: gtk::Box,
    pub center: gtk::Box,
    pub right: gtk::Box,

    pub rbar: Arc<RBar>,
}

impl Bar {
    pub fn new(app: &Application, rbar: Arc<RBar>) -> Self {
        let name = "rbar";

        let window = ApplicationWindow::builder().application(app).build();
        window.init_layer_shell();
        window.set_layer(gtk4_layer_shell::Layer::Top);

        window.style_context().add_class("bar");

        let content = gtk::CenterBox::builder()
            .orientation(Orientation::Horizontal)
            .hexpand(false)
            .height_request(rbar.config.bar.height)
            .name("bar")
            .build();

        content.style_context().add_class("content");

        let left = create_container("left");
        let center = create_container("center");
        let right = create_container("right");

        content.set_start_widget(Some(&left));
        content.set_center_widget(Some(&center));
        content.set_end_widget(Some(&right));

        window.set_child(Some(&content));

        window.connect_destroy(|_| {
            println!("destroy");
        });

        Self {
            name,
            window,
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
        use gtk4_layer_shell::{Edge, Layer};

        win.init_layer_shell();
        win.set_monitor(monitor);
        win.set_layer(Layer::Top);
        win.set_namespace(env!("CARGO_PKG_NAME"));

        win.auto_exclusive_zone_enable();

        let margin = &self.rbar.config.margin;
        win.set_margin(Edge::Top, margin.top);
        win.set_margin(Edge::Left, margin.left);
        win.set_margin(Edge::Right, margin.right);
        win.set_margin(Edge::Bottom, margin.bottom);

        win.set_anchor(Edge::Top, true);
        win.set_anchor(Edge::Left, true);
        win.set_anchor(Edge::Right, true);
    }

    fn show(&self) {
        self.window.show();
    }

    fn load_modules(&self) -> Result<()> {
        add_modules(&self)?;

        Ok(())
    }
}

pub fn load_css() {
    let style_path = env::current_dir().expect("to exist").join("style.css");

    let provider = CssProvider::new();
    provider.load_from_file(&gio::File::for_path(&style_path));

    let screen = Display::default().expect("Failed to get defautl GTK screen");
    gtk::style_context_add_provider_for_display(
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

    let monitors = display.monitors();

    for i in 0..monitors.n_items() {
        let monitor = monitors.item(i).expect("monitor to exist");
        // todo: add error handling
        let monitor = monitor.downcast::<Monitor>().unwrap();
        let _ = create_bar(app, rbar.clone(), &monitor);
    }
}

fn create_bar(app: &Application, rbar: Arc<RBar>, monitor: &Monitor) -> Result<Bar> {
    let bar = Bar::new(app, rbar);
    bar.init(monitor)
}

fn add_modules(bar: &Bar) -> Result<()> {
    let factory = ModuleFactory::new(bar.rbar.clone());

    for module in bar.rbar.config.bar.modules.iter() {
        module.create(&factory, bar)?;
    }

    Ok(())
}
