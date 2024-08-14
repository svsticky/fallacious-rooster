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
    #[error("File error: {0}")]
    DataFileError(#[from] crate::file::DataFileError),
    #[error("Advisor already exists")]
    AdvisorAlreadyExists,
    #[error("Advisor does not exist")]
    AdvisorDoesNotExist,
    #[error("Internal server error")]
    Internal,
    #[error("Unauthorized")]
    Unauthorized,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Email(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TemplateRender(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DataFileError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AdvisorAlreadyExists => StatusCode::CONFLICT,
            Self::AdvisorDoesNotExist => StatusCode::BAD_REQUEST,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
