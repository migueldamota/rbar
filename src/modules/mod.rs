use clock::Clock;
use gtk4::{glib, prelude::*, Application, Widget};
use tokio::sync::{broadcast, mpsc};

use std::{fmt::Debug, sync::Arc};

use crate::rbar::RBar;

pub mod clock;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum ModuleUpdateEvent<S: Clone> {
    Update(S),
}

pub struct WidgetContext<S, R>
where
    S: Clone,
{
    pub id: usize,
    pub rbar: Arc<RBar>,

    pub tx: mpsc::Sender<ModuleUpdateEvent<S>>,
    pub update_tx: broadcast::Sender<S>,
    pub controller_tx: mpsc::Sender<R>,
}

impl<S, R> WidgetContext<S, R>
where
    S: Clone,
{
    pub fn subscribe(&self) -> broadcast::Receiver<S> {
        self.update_tx.subscribe()
    }
}

pub trait Module<W>
where
    W: IsA<Widget>,
{
    type Send;
    type Receive;

    fn name() -> &'static str;

    fn spawn_controller(
        &self,
        context: &WidgetContext<Self::Send, Self::Receive>,
        rx: mpsc::Receiver<Self::Receive>,
    ) -> Result<()>
    where
        <Self as Module<W>>::Send: Clone;

    fn into_widget(self, context: WidgetContext<Self::Send, Self::Receive>) -> Result<W>
    where
        <Self as Module<W>>::Send: Clone;
}

pub trait ModuleFactory {
    fn create<M, W, S, R>(&self, module: M, container: &gtk4::Box) -> Result<()>
    where
        M: Module<W, Send = S, Receive = R>,
        W: IsA<Widget>,
        S: Debug + Clone + Send + 'static,
    {
        let id = RBar::unique_id();

        let (ui_tx, ui_rx) = mpsc::channel::<ModuleUpdateEvent<S>>(2);
        let (controller_tx, controller_rx) = mpsc::channel::<R>(2);

        let (tx, rx) = broadcast::channel(2);

        let context = WidgetContext {
            id,
            rbar: self.rbar().clone(),

            tx: ui_tx,
            update_tx: tx.clone(),
            controller_tx,
        };

        module.spawn_controller(&context, controller_rx)?;

        let module_name = M::name();

        let m = module.into_widget(context)?;
        m.set_widget_name(module_name);
        m.add_css_class("widget");

        self.setup_receiver(tx, ui_rx, module_name, id);

        container.append(&m);

        Ok(())
    }

    fn setup_receiver<S>(
        &self,
        tx: broadcast::Sender<S>,
        rx: mpsc::Receiver<ModuleUpdateEvent<S>>,
        name: &'static str,
        id: usize,
    ) where
        S: Debug + Clone + Send + 'static;

    fn rbar(&self) -> &Arc<RBar>;
}

pub struct BarModuleFactory {
    rbar: Arc<RBar>,
}

impl BarModuleFactory {
    pub fn new(rbar: Arc<RBar>) -> Self {
        Self { rbar }
    }
}

impl ModuleFactory for BarModuleFactory {
    fn rbar(&self) -> &Arc<RBar> {
        &self.rbar
    }

    fn setup_receiver<S>(
        &self,
        tx: broadcast::Sender<S>,
        rx: mpsc::Receiver<ModuleUpdateEvent<S>>,
        name: &'static str,
        id: usize,
    ) where
        S: Debug + Clone + Send + 'static,
    {
        glib::spawn_future_local(async move {
            let mut rx = rx;
            while let Some(ev) = rx.recv().await {
                match ev {
                    ModuleUpdateEvent::Update(data) => {
                        tx.send(data).expect("why tho");
                    }
                }
            }
        });
    }
}

pub enum Modules {
    Clock(Box<Clock>),
}

impl Modules {
    pub fn create(self, module_factory: &BarModuleFactory, container: &gtk4::Box) -> Result<()> {
        match self {
            Self::Clock(module) => module_factory.create(*module, container),
        }
    }
}
