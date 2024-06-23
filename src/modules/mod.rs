use glib::IsA;
use gtk::Widget;

use std::sync::Arc;

use crate::rbar::RBar;

mod clock;

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

    fn into_widget(self, context: &WidgetContext) -> Result<W>;
}
