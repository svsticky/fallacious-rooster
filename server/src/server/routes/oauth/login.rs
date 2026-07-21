use crate::server::types::{Error, WConfig, WResult};
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    location: String,
}

pub async fn login(config: WConfig) -> WResult<web::Json<Response>> {
    let location = config
        .oauth
        .get_authorization_url()
        .await
        .map_err(|_| Error::Internal)?;

    Ok(web::Json(Response { location }))
}
