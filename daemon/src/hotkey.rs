use anyhow::Result;
use rdev::{listen as rdev_listen, Event, EventType, Key};
use tracing::info;

use crate::config::Config;

pub async fn listen(config: Config) -> Result<()> {
    info!("hotkey listener started, watching for: {}", config.hotkey);

    let mut super_pressed = false;

    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(Key::MetaLeft) | EventType::KeyPress(Key::MetaRight) => {
                super_pressed = true;
            }
            EventType::KeyRelease(Key::MetaLeft) | EventType::KeyRelease(Key::MetaRight) => {
                super_pressed = false;
            }
            EventType::KeyPress(Key::KeyK) => {
                if super_pressed {
                    info!("Super+K detected — toggling overlay");
                }
            }
            _ => {}
        }
    };

    tokio::task::spawn_blocking(move || {
        rdev_listen(callback).ok();
    })
    .await?;

    Ok(())
}