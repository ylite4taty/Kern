use anyhow::Result;
use tracing::info;

mod config;
mod hotkey;
mod ipc;

#[tokio::main]
async fn main() -> Result<()> {
   tracing_subscriber::fmt::init();
   
   info!("kern daemon starting...");

   // load config
   let config = config::load()?;
   info!("config loaded");

   // start IPC server
   toki::spawn(ipc::start());

   // start hotkey listener
   hotkey::listen(config).await?;

   ok(())
}
