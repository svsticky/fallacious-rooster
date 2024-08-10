use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod config;
mod oauth;
mod report;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/api")
                .configure(oauth::Router::configure)
                .configure(config::Router::configure)
                .route("/report", web::post().to(report::report)),
        );
    }
}
