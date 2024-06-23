use gtk::prelude::{ButtonExt, WidgetExt};

use super::{Module, WidgetContext};

pub struct Clock {
    // /// Format: HH:MM:SS
    // #[serde(default = "%H:%M:%S")]
    // format: String,
}

impl Clock {
    pub fn new() -> Self {
        Self {}
    }
}

impl Module<gtk::Button> for Clock {
    fn name() -> &'static str {
        "clock"
    }

    fn into_widget(self, _: WidgetContext) -> super::Result<gtk::Button> {
        let date = chrono::Local::now();

        println!("Hello");

        let button = gtk::Button::builder()
            .label(format!("Hello {}", date.format("%H:%M:%S")))
            .build();

        button.show();

        Ok(button)
    }
}
