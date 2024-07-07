use std::{ops::Mul, time::Duration};

use gtk::{glib, prelude::*, Box, Label};
use serde::Deserialize;
use tracing::error;

use crate::rbar::RBar;

use super::{BaseModuleConfig, Events, Module};

#[derive(Debug, Deserialize)]
pub struct Power {
    config: BaseModuleConfig,

    /// Precision of the percentage.
    #[serde(default = "precision_default")]
    precision: u8,
}

impl Module<Box> for Power {
    type Receive = ();
    type Send = battery::Battery;

    fn name() -> &'static str {
        "power"
    }

    fn controllers(&self, context: &super::WidgetContext<Self::Send>) -> crate::Result<()> {
        let manager = battery::Manager::new();

        let mut battery = manager
            .batteries()?
            .into_iter()
            .next()
            .ok_or("Failed to get battery")?;

        let tx = context.tx.clone();
        RBar::runtime().spawn(async move {
            let duration = Duration::from_secs(1);

            while let Ok(battery) = battery.refresh() {
                if let Err(e) = tx.send(Events::Update(battery.clone())).await {
                    error!("Failed to send battery update: {}", e);
                    break;
                }

                tokio::time::sleep(duration).await;
            }
        });

        Ok(())
    }

    fn widget(&self, context: super::WidgetContext<Self::Send>) -> crate::Result<Box> {
        let container = Box::new(gtk::Orientation::Horizontal, 0);
        let icon = Label::new(None);
        let label = Label::new(None);

        icon.add_css_class("icon");
        label.add_css_class("label");

        container.append(&icon);
        container.append(&label);
        container.show();

        let precision = self.precision;

        let mut rx = context.subscribe();
        glib::spawn_future_local(async move {
            let classes = ["charging", "discharging", "full", "unknown"];

            let container = label.parent().unwrap();

            while let Ok(battery) = rx.recv().await {
                use battery::State;

                let class_index = match battery.state() {
                    State::Charging => 0,
                    State::Discharging => 1,
                    State::Full => 2,
                    _ => 3,
                };

                for (i, class) in classes.iter().enumerate() {
                    container.remove_css_class(class);
                    if i == class_index {
                        container.add_css_class(class);
                    }
                }

                format_label(&icon, &label, &battery, precision);
            }
        });

        Ok(container)
    }

    fn get_base_config(&self) -> &BaseModuleConfig {
        &self.config
    }
}

fn precision_default() -> u8 {
    0
}

fn format_label(icon: &Label, label: &Label, battery: &battery::Battery, precision: u8) {
    let soc = battery.state_of_charge();

    let icon_text = if battery.is_charging() {
        ""
    } else {
        match soc {
            0.0..=10.0 => "",
            11.0..=40.0 => "",
            41.0..=60.0 => "",
            61.0..=80.0 => "",
            81.0..=100.0 => "",
            _ => "",
        }
    };

    icon.set_label(icon_text);

    let label_text = format!("{:.1$}%", soc, precision as usize);

    label.set_label(&label_text);
}
