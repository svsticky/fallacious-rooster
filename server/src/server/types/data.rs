use crate::file::{AppConfig, AppStorage};
use actix_web::web;

pub type WConfig = web::Data<AppConfig>;
pub type WStorage = web::Data<AppStorage>;
