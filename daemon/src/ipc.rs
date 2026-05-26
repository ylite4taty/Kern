use anyhow::Result;
use tracing::info;


pub async fn start() -> Result<()> {
    info!("IPC server started");

    // placeholder: Unix socket server will be implemented here 
    // the daemon will listen for messages from the overlay
    // and respond with search results 
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

    }

}
