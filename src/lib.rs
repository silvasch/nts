use anyhow::Result;
use axum::{routing::get, Router};

pub async fn run() -> Result<()> {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9112").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
