use serde::{Deserialize, Serialize};

use crate::file::DataFile;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AppStorage {
    pub board: String,
    pub confidential_advisors: Vec<String>,
}

impl DataFile for AppStorage {}
