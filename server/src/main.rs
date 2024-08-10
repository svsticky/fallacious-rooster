use crate::args::AppArgs;
use crate::file::{AppConfig, AppStorage, DataFile};
use clap::Parser;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod args;
mod email;
mod file;
mod server;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    init_tracing()?;

    let args = AppArgs::parse();
    let config = AppConfig::try_read(&args.config).await?;
    let local_storage = AppStorage::try_read(&config.local_storage).await?;

    server::run_server(config, local_storage).await
}

fn init_tracing() -> color_eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(layer().compact())
        .with(tracing_error::ErrorLayer::default())
        .try_init()?;

    Ok(())
}
