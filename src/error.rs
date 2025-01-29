#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read the body of the request: {0}")]
    BodyRead(std::io::Error),

    #[error("config error: {0}")]
    Settings(#[from] config::ConfigError),
    #[error("xdg error: {0}")]
    Xdg(#[from] xdg::BaseDirectoriesError),
}
