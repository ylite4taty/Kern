use anyhow::Result;
use tracing::info;

use crate::config::Config;


pub async fn listen(config: Config) -> Result<()> {
    info!("hotkey listener started, waiting for: {}", config.hotkey);


    // placeholder: hotkey detection will be implemenented 
    // with a Linux-compatible global hotkey library
    // candidates: rdev, x11rb, evdev
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

    }
}

