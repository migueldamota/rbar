use chrono::{DateTime, Local};
use gtk::{glib, prelude::*, Button, Label};
use log::error;
use serde::Deserialize;
use tokio::time::{sleep, Duration};

use crate::rbar::RBar;

use super::{BaseModuleConfig, Events, Module, WidgetContext};

#[derive(Debug, Deserialize)]
pub struct Clock {
    config: BaseModuleConfig,

    /// Clock format
    /// Default: `%a %d/%m/%Y - %H:%M:%S %p`
    /// Example: Wed 01/01/2022 - 00:00:00 AM
    ///
    /// Formatting is based on [chrono::format::strftime](https://docs.rs/chrono/0.4/chrono/format/strftime/index.html).
    #[serde(default = "default_format")]
    pub format: String,
}

impl Module<Button> for Clock {
    type Receive = ();
    type Send = DateTime<Local>;

    fn name() -> &'static str {
        "clock"
    }

    fn controllers(&self, context: &WidgetContext<Self::Send>) -> crate::Result<()> {
        let tx = context.tx.clone();

        RBar::runtime().spawn(async move {
            let duration = Duration::from_millis(500);
            loop {
                let date = Local::now();
                if let Err(e) = tx.send(Events::Update(date)).await {
                    error!("Error while sending date: {}", e);
                    break;
                }

                sleep(duration).await;
            }
        });

        Ok(())
    }

    fn widget(&self, context: WidgetContext<Self::Send>) -> crate::Result<Button> {
        let date = Local::now();

        let button = Button::new();
        let label = Label::builder()
            .label(date.format(&self.format).to_string())
            .build();

        button.set_child(Some(&label));
        button.show();

        let format = self.format.clone();

        let mut rx = context.subscribe();
        glib::spawn_future_local(async move {
            while let Ok(date) = rx.recv().await {
                let formatted_date = date.format(&format).to_string();
                label.set_label(&formatted_date);
            }
        });

        Ok(button)
    }

    fn get_base_config(&self) -> &BaseModuleConfig {
        &self.config
    }
}

fn default_format() -> String {
    "%a %d/%m/%Y - %H:%M:%S %p".to_string()
}
