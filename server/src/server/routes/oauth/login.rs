use crate::server::types::{Error, WConfig, WResult};
use actix_web::web;
use cabbage::oauth::ClientConfig;
use cabbage::KoalaApi;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    location: String,
}

pub async fn login(config: WConfig) -> WResult<web::Json<Response>> {
    let koala_api = KoalaApi::new(config.koala.koala_host.clone()).map_err(|_| Error::Internal)?;
    let koala_auth = koala_api.oauth_api(ClientConfig::new(
        config.koala.client_id.clone(),
        config.koala.client_secret.clone(),
        config.koala.redirect_uri.clone(),
    ));

    Ok(web::Json(Response {
        location: koala_auth.get_login_redirect_uri(),
    }))
}
