use actix_web::web;
use serde::Deserialize;

use crate::file::DataFile;
use crate::server::types::{Authorization, Empty, Error, WConfig, WResult, WStorage};

#[derive(Debug, Deserialize)]
pub struct AddRequest {
    email: String,
}

pub async fn add(
    _: Authorization<true>,
    storage: WStorage,
    config: WConfig,
    payload: web::Json<AddRequest>,
) -> WResult<Empty> {
    let mut storage = storage.0.write().await;

    if storage.confidential_advisors.contains(&payload.email) {
        return Err(Error::AdvisorAlreadyExists);
    }

    storage
        .confidential_advisors
        .push(payload.into_inner().email);
    storage.try_write(&config.local_storage).await?;

    Ok(Empty)
}
