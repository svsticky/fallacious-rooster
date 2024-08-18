use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

mod callback;
mod login;
mod ok;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/oauth")
                .route("/callback", web::get().to(callback::callback))
                .route("/login", web::get().to(login::login))
                .route("/ok", web::get().to(ok::ok)),
        );
    }
}
