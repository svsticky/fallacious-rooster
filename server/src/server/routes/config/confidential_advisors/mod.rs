use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod add;
mod list;
mod remove;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/confidential-advisors")
                .route("", web::post().to(add::add))
                .route("", web::get().to(list::list))
                .route("", web::delete().to(remove::remove)),
        );
    }
}
