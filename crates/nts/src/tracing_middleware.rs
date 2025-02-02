use axum::{extract::Request, middleware::Next, response::Response};

const TARGET: &str = "requests";

pub async fn tracing_middleware(request: Request, next: Next) -> Response {
    tracing::debug!(target: TARGET, "{:?}", request);
    let response = next.run(request).await;
    tracing::debug!(target: TARGET, "{:?}", response);
    response
}
