use actix_web::web;
use serde::Deserialize;

use crate::file::{ConfidentialAdvisor, DataFile};
use crate::server::types::{Authorization, Empty, Error, WConfig, WResult, WStorage};

#[derive(Debug, Deserialize)]
pub struct AddRequest {
    name: String,
    email: String,
}

pub async fn add(
    _: Authorization<true>,
    storage: WStorage,
    config: WConfig,
    payload: web::Json<AddRequest>,
) -> WResult<Empty> {
    let mut storage = storage.0.write().await;

    if storage
        .confidential_advisors
        .iter()
        .find(|adv| adv.email.eq(&payload.email))
        .is_some()
    {
        return Err(Error::AdvisorAlreadyExists);
    }

    let payload = payload.into_inner();
    storage.confidential_advisors.push(ConfidentialAdvisor {
        name: payload.name,
        email: payload.email,
    });
    storage.try_write(&config.local_storage).await?;

    Ok(Empty)
}
