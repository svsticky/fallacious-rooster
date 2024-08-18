use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::file::DataFile;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub koala: KoalaConfig,
    pub email: EmailConfig,
    pub frontend: FrontendConfig,
    pub local_storage: PathBuf,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct FrontendConfig {
    pub home_page_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct KoalaConfig {
    pub koala_host: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct EmailConfig {
    pub from_email: String,
    pub from_name: String,
    pub smtp_relay: String,
}

fn default_port() -> u16 {
    8080
}

impl DataFile for AppConfig {}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
        }
    }
}
