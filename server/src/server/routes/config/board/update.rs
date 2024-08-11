use crate::file::DataFile;
use crate::server::types::{Authorization, Empty, WConfig, WResult, WStorage};
use actix_web::web;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    email: String,
}

pub async fn update(
    _: Authorization<true>,
    storage: WStorage,
    config: WConfig,
    payload: web::Json<UpdateRequest>,
) -> WResult<Empty> {
    let mut storage = storage.0.write().await;

    storage.board = payload.into_inner().email;
    storage.try_write(&config.local_storage).await?;

    Ok(Empty)
}
