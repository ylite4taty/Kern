use anyhow::Result;
use tracing::info;
use kern_engine::{KernIndex, SearchEngine};

mod config;
mod hotkey;
mod ipc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("kern daemon starting...");

    let config = config::load()?;
    info!("config loaded");

    // initialise and populate the index
    let kern_index = KernIndex::new(&config.index_path)?;
    kern_index.index_docs(&config.docs_path)?;
    info!("docs indexed");

    // build search engine
    let _engine = SearchEngine::new(kern_index);
    info!("search engine ready");

    // start IPC server
    tokio::spawn(ipc::start());

    // start hotkey listener
    hotkey::listen(config).await?;

    Ok(())
}