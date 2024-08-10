use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod board;
mod confidential_advisors;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/config")
                .configure(board::Router::configure)
                .configure(confidential_advisors::Router::configure),
        );
    }
}
