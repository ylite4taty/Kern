use anyhow::Result;
use kern_engine::{KernIndex, SearchEngine};
use std::sync::Arc;
use tracing::info;

mod config;
mod hotkey;
mod ipc;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("kern daemon starting...");

    let config = config::load()?;
    info!("config loaded");

    let kern_index = KernIndex::new(&config.index_path)?;
    kern_index.index_docs(&config.docs_path)?;
    info!("docs indexed");

    let engine = Arc::new(SearchEngine::new(kern_index));
    info!("search engine ready");

    let engine_ipc = Arc::clone(&engine);
    tokio::spawn(async move {
        if let Err(e) = ipc::start(engine_ipc).await {
            tracing::error!("IPC error: {}", e);
        }
    });

    let engine_http = Arc::clone(&engine);
    tokio::spawn(async move {
        if let Err(e) = server::start(engine_http).await{
            tracing::error!("HTTP server error: {}", e);
        }
    });

    hotkey::listen(config).await?;

    Ok(())
}