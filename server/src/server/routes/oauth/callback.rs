use crate::oauth::OAuthError;
use crate::server::types::{Error, Redirect, SetCookie, WConfig, WResult};
use actix_web::web;
use serde::Deserialize;
use tracing::trace;

#[derive(Debug, Deserialize)]
pub struct Query {
    code: String,
}

pub async fn callback(config: WConfig, query: web::Query<Query>) -> WResult<SetCookie<Redirect>> {
    let tokens = config
        .oauth
        .exchange_code(&query.code)
        .await
        .map_err(|e| match e {
            OAuthError::Unauthorized => Error::Unauthorized,
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
