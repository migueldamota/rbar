use std::ops::Mul;

use gtk::{glib, prelude::*, Box, Button, Label};
use serde::Deserialize;
use tokio::time::{sleep, Duration};
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
        let button = Button::new();
        let label = Label::builder().label("test").build();

        button.set_child(Some(&label));
        container.append(&button);
        container.show();

        let precision = self.precision;

        let mut rx = context.subscribe();
        glib::spawn_future_local(async move {
            let manager = match battery::Manager::new() {
                Ok(manager) => manager,
                Err(e) => {
                    error!("Failed to create battery manager: {}", e);
                    return;
                }
            };

            let mut battery = match manager
                .batteries()
                .and_then(|mut batteries| batteries.next().unwrap())
            {
                Ok(battery) => battery,
                Err(e) => {
                    error!("Failed to get battery: {}", e);
                    return;
                }
            };

            fn format_label(battery: &battery::Battery, precision: u8) -> String {
                use battery::State;

                let state = match battery.state() {
                    State::Charging => "Charging",
                    State::Discharging => "Discharging",
                    State::Empty => "Empty",
                    State::Full => "Full",
                    State::Unknown => "Unknown",
                    _ => "Unknown",
                };

                format!(
                    "{} | {:.p$}%",
                    state,
                    battery.state_of_charge().value.mul(100.0),
                    p = precision as usize,
                )
            }

            while rx.recv().await.is_ok() {
                match manager.refresh(&mut battery) {
                    Ok(_) => {
                        label.set_label(&format_label(&battery, precision));
                    }
                    Err(e) => error!("Failed to refresh battery: {}", e),
                }
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
