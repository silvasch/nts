use std::path::{Path, PathBuf};

use anyhow::Result;
use tracing::info;

const TARGET: &str = "state";

#[derive(Clone)]
pub struct State {
    pub data_dir: PathBuf,

    pub password: Option<String>,
}

impl State {
    pub fn new() -> Result<Self> {
        let data_dir = std::env::var("NTS_DATA_DIR")
            .map(|raw_data_dir| Path::new(&raw_data_dir).to_path_buf())
            .unwrap_or(xdg::BaseDirectories::with_prefix("nts")?.get_data_home());

        info!(target: TARGET, "data_dir = {}", data_dir.display());

        let password = std::fs::read_to_string(data_dir.join("pwd"))
            .ok()
            .map(|v| v.trim().to_string());

        info!(target: TARGET, "password = {:?}", password.clone().map(|_| "..."));

        Ok(Self { data_dir, password })
    }
}
