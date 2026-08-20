use crate::oauth::OAuthError;
use crate::server::types::WConfig;
use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpRequest, ResponseError};
use std::future::Future;
use std::pin::Pin;
use thiserror::Error;
use tracing::warn;

pub struct Authorization<const ADMIN: bool = false> {
    pub is_admin: bool,
}

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("Authorization token not provided or token invalid")]
    NoToken,
    #[error("Failed to validate authorization token with OAuth provider")]
    OAuth,
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
            let token = match get_token(&req) {
                Some(token) => token,
                None => {
                    return if Self::ADMIN {
                        Err(AuthorizationError::NoToken)
                    } else {
                        Ok(Self { is_admin: false })
                    }
                }
            };

            let config: &WConfig = req.app_data().unwrap();

            let userinfo = match config.oauth.get_userinfo(&token).await {
                Ok(userinfo) => userinfo,
                Err(e) => {
                    warn!("Failed to fetch userinfo from OAuth provider: {e:?}");
                    return match e {
                        OAuthError::Unauthorized => {
                            if Self::ADMIN {
                                Err(AuthorizationError::NoToken)
                            } else {
                                Ok(Self { is_admin: false })
                            }
                        }
                        _ => {
                            if Self::ADMIN {
                                Err(AuthorizationError::OAuth)
                            } else {
                                Ok(Self { is_admin: false })
                            }
                        }
                    };
                }
            };

            if Self::ADMIN && !userinfo.is_admin {
                return Err(AuthorizationError::Forbidden);
            }

            Ok(Self {
                is_admin: userinfo.is_admin,
            })
        })
    }
}

fn get_token(req: &HttpRequest) -> Option<String> {
    // Get the authorization from the Authorization header or an Authorization cookie
    let value = match header(req, "Authorization") {
        Some(header_value) => header_value,
        None => {
            let cookie = req.cookie("Authorization")?;
            cookie.value().to_string()
        }
    };

    let decoded = percent_encoding::percent_decode_str(&value)
        .decode_utf8()
        .ok()
        .map(|s| s.to_string())
        .unwrap_or(value);

    let trimmed = decoded.trim().trim_matches('"');

    if trimmed.len() >= 7 && trimmed[..7].eq_ignore_ascii_case("Bearer ") {
        Some(trimmed[7..].trim().to_string())
    } else if !trimmed.is_empty() && !trimmed.contains(' ') {
        Some(trimmed.to_string())
    } else {
        None
    }
}

fn header(req: &HttpRequest, name: &str) -> Option<String> {
    req.headers()
        .get(name)
        .and_then(|hv| hv.to_str().ok())
        .map(|v| v.to_string())
}

impl ResponseError for AuthorizationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NoToken => StatusCode::UNAUTHORIZED,
            Self::OAuth => StatusCode::BAD_GATEWAY,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Forbidden => StatusCode::FORBIDDEN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[test]
    fn test_get_token_header() {
        let req = TestRequest::default()
            .insert_header(("Authorization", "Bearer sample_token_123"))
            .to_http_request();
        assert_eq!(get_token(&req), Some("sample_token_123".to_string()));
    }

    #[test]
    fn test_get_token_cookie_encoded() {
        let req = TestRequest::default()
            .cookie(actix_web::cookie::Cookie::new(
                "Authorization",
                "Bearer%20sample_token_456",
            ))
            .to_http_request();
        assert_eq!(get_token(&req), Some("sample_token_456".to_string()));
    }
}
