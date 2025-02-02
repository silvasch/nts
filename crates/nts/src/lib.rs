use anyhow::Result;
use axum::{http::StatusCode, middleware, response::IntoResponse, routing::get, Router};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};

mod state;
pub(crate) use state::State;

mod tracing_middleware;
pub(crate) use tracing_middleware::tracing_middleware;

pub async fn run() -> Result<()> {
    let state = State::new()?;

    let app = Router::new()
        .route("/", get(hello))
        .layer(middleware::from_fn(tracing_middleware))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9112").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn hello(
    axum::extract::State(state): axum::extract::State<State>,
    creds: Option<TypedHeader<Authorization<Basic>>>,
) -> impl IntoResponse {
    if let Some(password_hash) = state.password {
        match creds {
            Some(creds) if creds.password().trim() == password_hash => (StatusCode::OK, "ok"),
            Some(_) => (StatusCode::UNAUTHORIZED, "invalid password"),
            None => (StatusCode::UNAUTHORIZED, "unauthorized"),
        }
    } else {
        (StatusCode::OK, "ok")
    }
}
