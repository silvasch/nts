use anyhow::Result;
use axum::{
    http::StatusCode,
    middleware,
    routing::{get, post},
    Router,
};
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
        .route("/api/check-pwd", get(check_password))
        .route("/api/new", post(new_note))
        .layer(middleware::from_fn(tracing_middleware))
        .with_state(state);

    let address = get_address()?;
    tracing::info!("binding to {}", address);
    let listener = tokio::net::TcpListener::bind(get_address()?).await?;

    axum::serve(listener, app).await?;
    Ok(())
}

async fn new_note(
    state: axum::extract::State<State>,
    creds: Option<TypedHeader<Authorization<Basic>>>,
    body: String,
) -> (StatusCode, String) {
    let (status_code, msg) = check_password(state.clone(), creds).await;

    if status_code != StatusCode::OK {
        return (status_code, msg.to_string());
    }

    let notes_dir = state.0.data_dir.join("notes");

    let file_name = state
        .0
        .data_dir
        .join("notes")
        .join(jiff::Timestamp::now().as_millisecond().to_string())
        .with_extension("txt");

    if let Err(e) = std::fs::create_dir_all(notes_dir) {
        tracing::error!("{:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to create notes directory".to_string(),
        );
    }

    if let Err(e) = std::fs::write(file_name, body) {
        tracing::error!("{:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to write note.".to_string(),
        );
    }

    (StatusCode::OK, "ok".to_string())
}

async fn check_password(
    axum::extract::State(state): axum::extract::State<State>,
    creds: Option<TypedHeader<Authorization<Basic>>>,
) -> (StatusCode, &'static str) {
    if let Some(password_hash) = state.password_hash {
        match creds {
            Some(creds) if creds.password().trim() == password_hash => (StatusCode::OK, "ok"),
            Some(_) => (StatusCode::UNAUTHORIZED, "invalid password"),
            None => (StatusCode::UNAUTHORIZED, "unauthorized"),
        }
    } else {
        (StatusCode::OK, "ok")
    }
}

fn get_address() -> Result<String> {
    let host = std::env::var("NTS_HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("NTS_PORT")
        .unwrap_or("9112".to_string())
        .parse::<u16>()?;
    Ok(format!("{}:{}", host, port))
}
