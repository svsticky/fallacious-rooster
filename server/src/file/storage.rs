use serde::{Deserialize, Serialize};

use crate::file::DataFile;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AppStorage {
    pub board: String,
    pub confidential_advisors: Vec<ConfidentialAdvisor>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfidentialAdvisor {
    pub name: String,
    pub email: String,
}

impl DataFile for AppStorage {}
