use gtk::prelude::ButtonExt;

use super::{Module, WidgetContext};

struct Clock {
    // /// Format: HH:MM:SS
    // #[serde(default = "%H:%M:%S")]
    // format: String,
}

impl Module<gtk::Button> for Clock {
    fn name() -> &'static str {
        "clock"
    }

    fn into_widget(self, context: &WidgetContext) -> super::Result<gtk::Button> {
        let date = chrono::Local::now();

        let button = gtk::Button::builder()
            .label(format!("{}", date.format("%H:%M:%S")))
            .build();

        Ok(button)
    }
}
