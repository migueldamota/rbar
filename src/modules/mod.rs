use std::{fmt::Debug, sync::Arc};

use gtk::{glib, prelude::*, Widget};
use serde::Deserialize;
use tokio::sync::{broadcast, mpsc};

use crate::{bar::Bar, RBar};

mod clock;

/// [WidgetContext] holds information about widget and rbar.
#[derive(Debug)]
pub struct WidgetContext<S: Clone> {
    pub id: usize,
    pub rbar: Arc<RBar>,

    pub tx: mpsc::Sender<Events<S>>,
    pub update_tx: broadcast::Sender<S>,
}

impl<S: Clone> WidgetContext<S> {
    /// Subscribe to the update channel of the module to receive updates and handle them.
    pub fn subscribe(&self) -> broadcast::Receiver<S> {
        self.update_tx.subscribe()
    }
}

pub trait Module<W: IsA<Widget>> {
    /// Data to be received from the module.
    type Receive;
    /// Data to be sent from the module.
    type Send: Clone + Debug + Send + 'static;

    /// Name of the module.
    /// Can be used to identify the module and for styling purposes.
    fn name() -> &'static str;

    /// Create controllers to handle certain events.
    fn controllers(&self, context: &WidgetContext<Self::Send>) -> crate::Result<()>;

    /// Create the widget. Return the widget itself.
    fn widget(self, context: WidgetContext<Self::Send>) -> crate::Result<W>;

    /// Get module configuration.
    fn get_base_config(&self) -> &BaseModuleConfig;

    fn is_enabled(&self) -> bool {
        self.get_base_config().enabled
    }

    fn get_position(&self) -> &ModulePosition {
        &self.get_base_config().position
    }
}

/// [ModuleFactory] is creating instances of modules.
pub struct ModuleFactory {
    rbar: Arc<RBar>,
}

impl ModuleFactory {
    /// Create a new [ModuleFactory].
    pub fn new(rbar: Arc<RBar>) -> Self {
        Self { rbar }
    }

    /// Create a widget and adds it to the container.
    fn create<M, W>(&self, module: M, bar: &Bar) -> crate::Result<()>
    where
        M: Module<W>,
        W: IsA<Widget>,
    {
        let id = RBar::unique_id();

        let (ui_tx, ui_rx) = mpsc::channel::<Events<M::Send>>(32);

        let (tx, rx) = broadcast::channel(32);

        let context = WidgetContext {
            id,
            rbar: self.rbar.clone(),

            tx: ui_tx,
            update_tx: tx.clone(),
        };

        // Create controllers.
        module.controllers(&context)?;

        // Get container.
        let container = match module.get_position() {
            ModulePosition::Left => &bar.left,
            ModulePosition::Center => &bar.center,
            ModulePosition::Right => &bar.right,
        };

        // Create widget.
        let widget = module.widget(context)?;
        widget.set_widget_name(M::name());
        widget.add_css_class("widget");
        widget.add_css_class(M::name());

        // Append widget to container.
        container.append(&widget);

        // Setup receiver for module updates (and other events).
        self.setup_receiver(tx, ui_rx);

        Ok(())
    }

    fn setup_receiver<S: Debug + Clone + Send + 'static>(
        &self,
        tx: broadcast::Sender<S>,
        mut rx: mpsc::Receiver<Events<S>>,
    ) {
        glib::spawn_future_local(async move {
            while let Some(event) = rx.recv().await {
                use Events::*;
                match event {
                    Update(data) => {
                        // todo: handle error
                        tx.send(data).expect("Handle error!!!");
                    }
                }
            }
        });
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum Modules {
    Clock(clock::Clock),
}

impl Modules {
    pub fn create(self, module_factory: &ModuleFactory, bar: &Bar) -> crate::Result<()> {
        macro_rules! create {
            ($module:expr) => {
                module_factory.create($module, bar)
            };
        }

        match self {
            Self::Clock(module) => create!(module),
        }
    }
}

pub enum Events<S: Clone> {
    /// Modules updates.
    Update(S),
}

#[derive(Debug, Deserialize)]
pub struct BaseModuleConfig {
    pub enabled: bool,
    pub position: ModulePosition,
}

/// [ModulePosition] is used to get the container wher the module should be added.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModulePosition {
    Left,
    Center,
    Right,
}
