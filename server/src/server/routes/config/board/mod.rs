use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod get;
mod update;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/board")
                .route("", web::get().to(get::get))
                .route("", web::patch().to(update::update)),
        );
    }
}
