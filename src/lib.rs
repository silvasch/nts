use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};

pub async fn run() -> Result<()> {
    let app = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9112").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn hello(creds: Option<TypedHeader<Authorization<Basic>>>) -> impl IntoResponse {
    match creds {
        Some(creds) if creds.password().trim_end() == "pwd" => (StatusCode::OK, "ok"),
        Some(_) => (StatusCode::UNAUTHORIZED, "invalid password"),
        None => (StatusCode::UNAUTHORIZED, "unauthorized"),
    }
}
