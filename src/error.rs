#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("config error: {0}")]
    Settings(#[from] config::ConfigError),
}
