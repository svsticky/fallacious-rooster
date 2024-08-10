use color_eyre::eyre::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod config;
mod storage;

pub use config::*;
pub use storage::*;

pub trait DataFile: Default + Serialize + DeserializeOwned {
    async fn try_read<P: AsRef<Path>>(from: P) -> color_eyre::Result<Self> {
        let from = from.as_ref();

        if !from.exists() {
            Self::try_write_new(from).await?;
            return Err(Error::msg(format!(
                "File does not exist. I have created a new configuration empty file at {from:?}"
            )));
        }

        let mut f = fs::File::open(from).await?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await?;

        Ok(serde_json::from_slice(&buf)?)
    }

    async fn try_write_new<P: AsRef<Path>>(to: P) -> color_eyre::Result<()> {
        let contents = serde_json::to_vec_pretty(&Self::default())?;

        let mut f = fs::File::create(to).await?;
        f.write_all(&contents).await?;

        Ok(())
    }
}
