use gtk::{
    gdk::{Display, Monitor},
    prelude::*,
    Application, ApplicationWindow, Orientation,
};
use gtk4_layer_shell::{Layer, LayerShell};
use tracing::{debug, error};

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
    /// Create a new bar.
    pub fn create(app: &Application, rbar: Arc<RBar>, monitor: &Monitor) -> Result<Self> {
        let bar = Bar::new(app, rbar);
        bar.init(monitor)
    }

    fn new(app: &Application, rbar: Arc<RBar>) -> Self {
        let name = "rbar";

        let window = ApplicationWindow::builder().application(app).build();
        window.init_layer_shell();
        window.set_layer(Layer::Top);

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
            debug!("destroy");
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
        debug!(
            "Initializing bar '{}' on {:?}",
            self.name,
            monitor.manufacturer()
        );

        self.setup_layer_shell(&self.window, monitor);

        self.load_modules()?;

        self.show();

        Ok(self)
    }

    fn setup_layer_shell(&self, win: &ApplicationWindow, monitor: &Monitor) {
        use gtk4_layer_shell::Edge;

        win.init_layer_shell();
        win.set_monitor(monitor);
        win.set_layer(Layer::Background);
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
        let factory = ModuleFactory::new(self.rbar.clone());

        for module in self.rbar.config.bar.modules.iter() {
            module.create(&factory, self)?;
        }

        Ok(())
    }
}

fn create_container(name: &str) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .name(name)
        .build();

    let style_context = container.style_context();
    style_context.add_class("container");
    style_context.add_class(name);

    container
}

fn get_display() -> Display {
    use std::process::exit;

    Display::default().map_or_else(|| exit(3), |display| display)
}

pub fn load_bars(rbar: Arc<RBar>, app: &Application) -> Result<()> {
    let display = get_display();

    let monitors = display.monitors();

    for i in 0..monitors.n_items() {
        let monitor = monitors.item(i).expect("monitor to exist");
        // todo: add error handling
        let monitor = match monitor.downcast::<Monitor>() {
            Ok(monitor) => monitor,
            Err(e) => {
                error!("Failed to downcast monitor: {:#?}", e);
                continue;
            }
        };
        Bar::create(app, rbar.clone(), &monitor)?;
    }

    Ok(())
}
