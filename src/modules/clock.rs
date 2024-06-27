use gtk4::{prelude::*, Button, Label};

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

impl Module<Button> for Clock {
    fn name() -> &'static str {
        "clock"
    }

    fn into_widget(self, _: WidgetContext) -> super::Result<Button> {
        let format = "%a %d/%m/%Y - %H:%M:%S";
        let date = chrono::Local::now();

        let button = Button::new();
        let label = Label::builder()
            .label(format!("{}", date.format(format)))
            .build();

        label.show();
        button.set_child(Some(&label));

        button.show();

        Ok(button)
    }
}
