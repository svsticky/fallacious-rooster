use crate::server::types::{Authorization, WResult, WStorage};
use actix_web::web;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetResponse {
    email: String,
}

pub async fn get(_: Authorization<true>, storage: WStorage) -> WResult<web::Json<GetResponse>> {
    Ok(web::Json(GetResponse {
        email: storage.0.read().await.board.clone(),
    }))
}
