use std::ops::Mul;

use gtk::{glib, prelude::*, Box, Label};
use serde::Deserialize;
use tokio::{
    sync::broadcast,
    time::{sleep, Duration},
};
use tracing::error;

use crate::rbar::RBar;

use super::{BaseModuleConfig, Events, Module};

#[derive(Debug, Deserialize)]
pub struct Battery {
    config: BaseModuleConfig,

    /// Precision of the percentage.
    #[serde(default = "precision_default")]
    precision: u8,
}

impl Module<Box> for Battery {
    type Receive = ();
    type Send = ();

    fn name() -> &'static str {
        "battery"
    }

    fn controllers(&self, context: &super::WidgetContext<Self::Send>) -> crate::Result<()> {
        let tx = context.tx.clone();
        let duration = Duration::from_secs(1);

        RBar::runtime().spawn(async move {
            loop {
                if tx.send(Events::Update(())).await.is_err() {
                    break;
                }
                sleep(duration).await;
            }
        });

        Ok(())
    }

    fn widget(&self, context: super::WidgetContext<Self::Send>) -> crate::Result<Box> {
        let container = Box::new(gtk::Orientation::Horizontal, 0);
        let icon = Label::new(Some(""));
        let label = Label::new(Some(""));

        icon.add_css_class("icon");
        label.add_css_class("label");

        container.append(&icon);
        container.append(&label);
        container.show();

        let precision = self.precision;

        let rx = context.subscribe();
        glib::spawn_future_local(async move {
            if let Err(e) = run_widget(rx, &icon, &label, precision).await {
                error!("Failed to run widget: {}", e);
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

async fn run_widget(
    mut rx: broadcast::Receiver<()>,
    icon: &Label,
    label: &Label,
    precision: u8,
) -> crate::Result<()> {
    let manager = battery::Manager::new().inspect_err(|_| {
        error!("Failed to create battery mannager");
    })?;

    let mut battery = manager
        .batteries()?
        .next()
        .ok_or("No batteries found")?
        .map_err(|e| {
            error!("Failed to get battery: {}", e);
            e
        })?;

    let container = label.parent().unwrap();
    let classes = ["charging", "discharging", "empty", "full", "unknown"];

    while rx.recv().await.is_ok() {
        if let Err(e) = manager.refresh(&mut battery) {
            error!("Failed to refresh battery: {}", e);
        } else {
            use battery::State;

            let class_index = match battery.state() {
                State::Charging => 0,
                State::Discharging => 1,
                State::Empty => 2,
                State::Full => 3,
                _ => 4,
            };

            for (i, class) in classes.iter().enumerate() {
                container.remove_css_class(class);
                if i == class_index {
                    container.add_css_class(class);
                }
            }

            format_label(icon, label, &battery, precision);
        }
    }

    Ok(())
}

fn format_label(icon: &Label, label: &Label, battery: &battery::Battery, precision: u8) {
    use battery::State;
    let soc = battery.state_of_charge().value.mul(100.0);

    let icon_text = if battery.state() == State::Charging {
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

    let label_text = format!(
        "{:.1$}%",
        battery.state_of_charge().value.mul(100.0),
        precision as usize,
    );

    label.set_label(&label_text);
}
