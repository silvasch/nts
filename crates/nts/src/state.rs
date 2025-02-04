use std::path::{Path, PathBuf};

use anyhow::Result;
use tracing::info;

const TARGET: &str = "state";

const NEW_SCRIPT: &str = include_str!("../../../scripts/new.sh");
const GET_SCRIPT: &str = include_str!("../../../scripts/get.sh");

#[derive(Clone)]
pub struct State {
    pub data_dir: PathBuf,

    pub password_hash: Option<String>,

    pub template: String,

    pub new_script: String,
    pub get_script: String,
}

impl State {
    pub fn new() -> Result<Self> {
        let data_dir = std::env::var("NTS_DATA_DIR")
            .map(|raw_data_dir| Path::new(&raw_data_dir).to_path_buf())
            .unwrap_or(xdg::BaseDirectories::with_prefix("nts")?.get_data_home());

        let password_hash = std::fs::read_to_string(data_dir.join("pwd"))
            .ok()
            .map(|v| v.trim().to_string());
        if password_hash.is_none() {
            info!("no password set; every request will be rejected. apply one using 'nts_set_pwd'");
        }

        let external_url =
            std::env::var("NTS_EXTERNAL_URL").unwrap_or("http://localhost:9112".to_string());

        let template = std::env::var("NTS_TEMPLATE").unwrap_or_default();

        let new_script = NEW_SCRIPT.replace("localhost:9112", &external_url);
        let get_script = GET_SCRIPT.replace("localhost:9112", &external_url);

        info!(target: TARGET, "data_dir = {}", data_dir.display());
        info!(target: TARGET, "password_hash = {:?}", password_hash);
        info!(target: TARGET, "external_url = {}", external_url);
        info!(target: TARGET, "template = {}", template);

        Ok(Self {
            data_dir,
            password_hash,
            template,
            new_script,
            get_script,
        })
    }
}
