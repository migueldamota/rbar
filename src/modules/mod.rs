use clock::Clock;
use gtk4::{prelude::*, Widget};

use std::sync::Arc;

use crate::rbar::RBar;

pub mod clock;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct WidgetContext {
    pub id: usize,
    pub rbar: Arc<RBar>,
}

pub trait Module<W>
where
    W: IsA<Widget>,
{
    fn name() -> &'static str;

    fn into_widget(self, context: WidgetContext) -> Result<W>;
}

pub trait ModuleFactory {
    fn create<M, W>(&self, module: M, container: &gtk4::Box) -> Result<()>
    where
        M: Module<W>,
        W: IsA<Widget>,
    {
        let id = RBar::unique_id();
        let context = WidgetContext {
            id,
            rbar: self.rbar().clone(),
        };

        let m = module.into_widget(context)?;
        m.set_widget_name("clock");

        container.append(&m);

        Ok(())
    }

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
