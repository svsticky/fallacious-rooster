use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;

pub type WResult<T> = Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to send email: {0}")]
    Email(#[from] crate::email::SendError),
    #[error("Failed to render email body: {0}")]
    TemplateRender(#[from] handlebars::RenderError),
    #[error("Invalid confidential advisors provided")]
    InvalidAdvisors,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Email(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TemplateRender(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidAdvisors => StatusCode::BAD_REQUEST,
        }
    }
}
