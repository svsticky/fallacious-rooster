use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::file::DataFile;
use crate::oauth::OAuthConfig;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    #[serde(alias = "koala", alias = "oidc")]
    pub oauth: OAuthConfig,
    pub email: EmailConfig,
    pub frontend: FrontendConfig,
    pub local_storage: PathBuf,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct FrontendConfig {
    pub home_page_url: String,
    pub domain: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    pub domain: String,
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
            domain: String::default(),
        }
    }
}
