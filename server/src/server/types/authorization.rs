use crate::server::types::WConfig;
use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpRequest, ResponseError};
use cabbage::oauth::ClientConfig;
use cabbage::KoalaApi;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use thiserror::Error;

pub struct Authorization<const ADMIN: bool = false> {
    pub full_name: String,
    pub is_admin: bool,
}

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("Authorization token not provided or token invalid")]
    NoToken,
    #[error("Failed to validate authorization token with Koala")]
    Koala,
    #[error("Internal server error")]
    Internal,
    #[error("Forbidden: Admin privileges are required.")]
    Forbidden,
}

impl<const ADMIN: bool> Authorization<ADMIN> {
    const ADMIN: bool = ADMIN;
}

impl<const ADMIN: bool> FromRequest for Authorization<ADMIN> {
    type Error = AuthorizationError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            #[cfg(debug_assertions)]
            if is_debug_allow_unauthorized(&req) {
                return Ok(Self {
                    full_name: "Debug Admin".to_string(),
                    is_admin: true,
                });
            }

            let token = match get_token(&req) {
                Some(token) => token,
                None => return Err(AuthorizationError::NoToken),
            };

            let config: &WConfig = req.app_data().unwrap();
            let koala_api = KoalaApi::new(config.koala.koala_host.clone())
                .map_err(|_| AuthorizationError::Internal)?;
            let koala_auth = koala_api.oauth_api(ClientConfig::new(
                config.koala.client_id.clone(),
                config.koala.client_secret.clone(),
                config.koala.redirect_uri.clone(),
            ));

            let userinfo = match koala_auth.get_userinfo(&token).await {
                Ok(userinfo) => userinfo,
                Err(e) => {
                    return match e.status() {
                        Some(StatusCode::UNAUTHORIZED) => Err(AuthorizationError::NoToken),
                        _ => Err(AuthorizationError::Koala),
                    }
                }
            };

            if Self::ADMIN && !userinfo.is_admin {
                return Err(AuthorizationError::Forbidden);
            }

            Ok(Self {
                full_name: userinfo.full_name,
                is_admin: userinfo.is_admin,
            })
        })
    }
}

/// Check if the `X-DebugAllowUnauthorized` header is present.
/// This is useful when working on the UI using the native Linux application,
/// which doesn't support browser redirects (obviously).
///
/// During devlopment ACL can be ignored this way.
#[cfg(debug_assertions)]
fn is_debug_allow_unauthorized(req: &HttpRequest) -> bool {
    match header(req, "X-DebugAllowUnauthorized") {
        Some(hv) => bool::from_str(&hv).unwrap_or(false),
        None => false,
    }
}

fn get_token(req: &HttpRequest) -> Option<String> {
    // Get the authorization from the Authorization header or an Authorization cookie
    let value = match header(req, "Authorization") {
        Some(header_value) => header_value,
        None => match req.cookie("Authorization") {
            Some(cookie) => cookie.value().to_string(),
            None => return None,
        },
    };

    if !value.starts_with("Bearer ") {
        return None;
    }

    Some(value.chars().skip(7).collect())
}

fn header(req: &HttpRequest, name: &str) -> Option<String> {
    req.headers()
        .get(name)
        .map(|hv| hv.to_str().ok())
        .flatten()
        .map(|v| v.to_string())
}

impl ResponseError for AuthorizationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NoToken => StatusCode::UNAUTHORIZED,
            Self::Koala => StatusCode::BAD_GATEWAY,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Forbidden => StatusCode::FORBIDDEN,
        }
    }
}
