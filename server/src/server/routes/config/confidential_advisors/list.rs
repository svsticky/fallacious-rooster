use crate::server::types::{Authorization, WResult, WStorage};
use actix_web::web;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListResponse {
    advisors: Vec<Advisor>,
}

#[derive(Debug, Serialize)]
pub struct Advisor {
    name: String,
    email: String,
}

pub async fn list(_: Authorization<true>, storage: WStorage) -> WResult<web::Json<ListResponse>> {
    Ok(web::Json(ListResponse {
        advisors: storage
            .0
            .read()
            .await
            .confidential_advisors
            .clone()
            .into_iter()
            .map(|adv| Advisor {
                name: adv.name,
                email: adv.email,
            })
            .collect(),
    }))
}
