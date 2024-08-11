use crate::server::types::{Authorization, WResult, WStorage};
use actix_web::web;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListResponse {
    advisors: Vec<String>,
}

pub async fn list(_: Authorization<true>, storage: WStorage) -> WResult<web::Json<ListResponse>> {
    Ok(web::Json(ListResponse {
        advisors: storage.0.read().await.confidential_advisors.clone(),
    }))
}
