use actix_web::web;
use serde::Deserialize;

use crate::file::DataFile;
use crate::server::types::{Authorization, Empty, Error, WConfig, WResult, WStorage};

#[derive(Debug, Deserialize)]
pub struct RemoveRequest {
    email: String,
}

pub async fn remove(
    _: Authorization<true>,
    storage: WStorage,
    config: WConfig,
    payload: web::Json<RemoveRequest>,
) -> WResult<Empty> {
    let mut storage = storage.0.write().await;

    if !storage
        .confidential_advisors
        .iter()
        .any(|adv| adv.email.eq(&payload.email))
    {
        return Err(Error::AdvisorDoesNotExist);
    }

    storage
        .confidential_advisors
        .retain(|advisor| advisor.email.ne(&payload.email));
    storage.try_write(&config.local_storage).await?;

    Ok(Empty)
}
