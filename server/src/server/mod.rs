use crate::args::AppArgs;
use crate::file::{AppConfig, AppStorage};
use crate::server::types::{MutAppStorage, WArgs, WConfig, WStorage};
use actix_cors::Cors;
use actix_route_config::Routable;
use actix_web::{App, HttpServer};
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use tokio::sync::RwLock;

mod routes;
mod types;

pub async fn run_server(
    config: AppConfig,
    storage: AppStorage,
    args: AppArgs,
) -> color_eyre::Result<()> {
    let port = config.server.port;

    let storage = WStorage::new(MutAppStorage(RwLock::new(storage)));
    let host = config.server.domain.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(tracing_actix_web::TracingLogger::<NoiselessRootSpanBuilder>::new())
            .app_data(WConfig::new(config.clone()))
            .app_data(storage.clone())
            .app_data(WArgs::new(args.clone()))
            .configure(routes::Router::configure)
    })
    .bind(format!("0.0.0.0:{port}"))?
    .server_hostname(&host)
    .run()
    .await?;

    Ok(())
}
