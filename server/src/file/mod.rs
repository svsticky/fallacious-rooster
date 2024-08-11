use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub use config::*;
pub use storage::*;

mod config;
mod storage;

#[derive(Debug, Error)]
pub enum DataFileError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("No file existed yet. Created new empty file.")]
    NewEmptyFileCreated,
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
}

pub trait DataFile: Default + Serialize + DeserializeOwned {
    async fn try_read<P: AsRef<Path>>(from: P) -> Result<Self, DataFileError> {
        let from = from.as_ref();

        if !from.exists() {
            Self::try_write_new(from).await?;
            return Err(DataFileError::NewEmptyFileCreated);
        }

        let mut f = fs::File::open(from).await?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await?;

        Ok(serde_json::from_slice(&buf)?)
    }

    async fn try_write_new<P: AsRef<Path>>(to: P) -> Result<(), DataFileError> {
        Self::default().try_write(to).await
    }

    async fn try_write<P: AsRef<Path>>(&self, to: P) -> Result<(), DataFileError> {
        let contents = serde_json::to_vec_pretty(self)?;

        let mut f = fs::File::create(to).await?;
        f.write_all(&contents).await?;

        Ok(())
    }
}
