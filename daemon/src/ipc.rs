use anyhow::Result;
use kern_engine::SearchEngine;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;
use tracing::info;

pub const SOCKET_PATH: &str = "/tmp/kern.sock";

#[derive(Deserialize)]
struct Query {
    query: String,
}

#[derive(Serialize)]
struct Response {
    results: Vec<String>,
}

pub async fn start(engine: Arc<SearchEngine>) -> Result<()> {
    if std::path::Path::new(SOCKET_PATH).exists() {
        std::fs::remove_file(SOCKET_PATH)?;
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;
    info!("IPC server listening at {}", SOCKET_PATH);

    loop {
        let (stream, _) = listener.accept().await?;
        let engine = Arc::clone(&engine);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, engine).await {
                tracing::warn!("IPC connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: tokio::net::UnixStream,
    engine: Arc<SearchEngine>,
) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    reader.read_line(&mut line).await?;
    let trimmed = line.trim();
    info!("IPC received: {}", trimmed);

    let results = match serde_json::from_str::<Query>(trimmed) {
        Ok(q) => engine.query(&q.query).unwrap_or_default(),
        Err(_) => vec!["invalid query format".to_string()],
    };

    let response = serde_json::to_string(&Response { results })? + "\n";
    writer.write_all(response.as_bytes()).await?;

    Ok(())
}