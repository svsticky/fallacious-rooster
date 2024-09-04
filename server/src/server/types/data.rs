use actix_web::web;
use tokio::sync::RwLock;
use crate::args::AppArgs;
use crate::file::{AppConfig, AppStorage};

pub type WConfig = web::Data<AppConfig>;

pub struct MutAppStorage(pub RwLock<AppStorage>);

pub type WStorage = web::Data<MutAppStorage>;

pub type WArgs = web::Data<AppArgs>;