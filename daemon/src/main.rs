use anyhow::Result;
use tracing::info;

mod config;
mod hotkey;
mod ipc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("kern daemon starting...");

    let config = config::load()?;
    info!("config loaded");

    tokio::spawn(ipc::start());

    hotkey::listen(config).await?;

    Ok(())
}