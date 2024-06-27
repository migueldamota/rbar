use chrono::{DateTime, Local};
use gtk4::{glib, prelude::*, Button, Label};
use log::error;
use tokio::time::sleep;

use crate::rbar::RBar;

use super::{Module, ModuleUpdateEvent, WidgetContext};

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
    type Receive = ();
    type Send = DateTime<Local>;

    fn name() -> &'static str {
        "clock"
    }

    fn spawn_controller(
        &self,
        context: &WidgetContext<Self::Send, Self::Receive>,
        _rx: tokio::sync::mpsc::Receiver<Self::Receive>,
    ) -> super::Result<()> {
        let tx = context.tx.clone();

        RBar::runtime().spawn(async move {
            let duration = std::time::Duration::from_millis(500);
            loop {
                let date = Local::now();
                // todo: add error handling or something!
                tx.send(ModuleUpdateEvent::Update(date)).await.unwrap();

                sleep(duration).await;
            }
        });

        Ok(())
    }

    fn into_widget(
        self,
        context: WidgetContext<Self::Send, Self::Receive>,
    ) -> super::Result<Button> {
        let format = "%a %d/%m/%Y - %H:%M:%S %p";
        let date = chrono::Local::now();

        let button = Button::new();
        let label = Label::builder()
            .label(format!("{}", date.format(format)))
            .build();

        button.set_child(Some(&label));
        button.show();

        let rx = context.subscribe();
        glib::spawn_future_local(async move {
            let mut rx = rx;
            loop {
                match rx.recv().await {
                    Ok(date) => {
                        let str = format!("{}", date.format(format));
                        label.set_label(&str);
                    }
                    Err(err) => {
                        error!("Error while loading date: {}", err);
                        break;
                    }
                }
            }
        });

        Ok(button)
    }
}
