use crate::server::types::{Error, Redirect, SetCookie, WConfig, WResult};
use actix_web::http::StatusCode;
use actix_web::web;
use cabbage::oauth::ClientConfig;
use cabbage::KoalaApi;
use serde::Deserialize;
use tracing::trace;

#[derive(Debug, Deserialize)]
pub struct Query {
    code: String,
}

pub async fn callback(config: WConfig, query: web::Query<Query>) -> WResult<SetCookie<Redirect>> {
    let koala_api = KoalaApi::new(config.koala.koala_host.clone()).map_err(|_| Error::Internal)?;
    let koala_auth = koala_api.oauth_api(ClientConfig::new(
        config.koala.client_id.clone(),
        config.koala.client_secret.clone(),
        config.koala.redirect_uri.clone(),
    ));

    let tokens = koala_auth
        .exchange_login_code(&query.code)
        .await
        .map_err(|e| match e.status() {
            Some(StatusCode::UNAUTHORIZED) => Error::Unauthorized,
            Some(StatusCode::BAD_REQUEST) => Error::Unauthorized,
            _ => {
                trace!("{e}");
                Error::Internal
            }
        })?;

    Ok(SetCookie::new(
        Redirect::new(&config.frontend.home_page_url),
        "Authorization",
        format!("Bearer {}", tokens.access_token),
    ))
}
