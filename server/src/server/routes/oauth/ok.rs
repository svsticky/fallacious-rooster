use crate::server::types::Authorization;
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    is_admin: bool,
}

pub async fn ok(auth: Authorization) -> web::Json<Response> {
    web::Json(Response {
        is_admin: auth.is_admin,
    })
}
