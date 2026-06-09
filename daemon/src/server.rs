use anyhow::Result;
use axum::{
    extract::State,
    http::Method,
    routing::post,
    Json, Router,
};
use kern_engine::SearchEngine;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[derive(Deserialize)]
pub struct SearchRequest {
    pub query: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub results: Vec<String>,
}

pub async fn start(engine: Arc<SearchEngine>) -> Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/search", post(handle_search))
        .layer(cors)
        .with_state(engine);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("HTTP server listening at http://127.0.0.1:3000");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_search(
    State(engine): State<Arc<SearchEngine>>,
    Json(payload): Json<SearchRequest>,
) -> Json<SearchResponse> {
    let results = engine.query(&payload.query).unwrap_or_default();
    Json(SearchResponse { results })
}