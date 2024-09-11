use crate::args::AppArgs;
use crate::file::{AppConfig, AppStorage};
use actix_web::web;
use std::net::Ipv4Addr;
use tokio::sync::RwLock;

pub type WConfig = web::Data<AppConfig>;

pub struct MutAppStorage(pub RwLock<AppStorage>);

pub type WStorage = web::Data<MutAppStorage>;

pub type WArgs = web::Data<AppArgs>;

pub type WRuntime = web::Data<RuntimeData>;

#[derive(Clone)]
pub struct RuntimeData {
    pub local_v4_addr: Ipv4Addr,
}
