pub mod rest;
pub mod rpc;

use crate::domain::AuthError;
use crate::service::ServiceError;
use actix_web::http::StatusCode;
use actix_web::{error, web, HttpResponse};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Bad Request: {0}")]
    BadRequest(#[from] AuthError),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized")]
    UnAuthorized,

    #[error("Not found")]
    NotFound,

    #[error("Request timeout")]
    RequestTimeout,

    #[error("Too many request")]
    TooManyRequest,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Forbidden error: {0}")]
    Forbidden(&'static str),

    #[error("Permission error")]
    PermissionError,

    #[error("Bad Request error")]
    BadRequestError,
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::AuthError(text) => ApiError::BadRequest(text),
            ServiceError::InvalidCredentials => ApiError::InvalidCredentials,
            ServiceError::NotFound => ApiError::NotFound,
            ServiceError::UnAuthorized => ApiError::UnAuthorized,
            ServiceError::PermissionError(_) => ApiError::PermissionError,
            ServiceError::Forbidden(text) => ApiError::Forbidden(text),
            ServiceError::BadRequestError => ApiError::BadRequestError,
            _ => ApiError::InternalServerError,
        }
    }
}

#[derive(Serialize)]
pub struct SuccessResponseToClient<T> {
    status: String,
    message: T,
}

impl<T> SuccessResponseToClient<T> {
    pub fn new(message: T) -> Self {
        Self {
            status: "success".to_string(),
            message,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponseToClient {
    status: String,
    message: String,
}

impl ErrorResponseToClient {
    pub fn new(status: String, message: String) -> Self {
        Self { status, message }
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponseToClient {
            status: "error".to_string(),
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(web::Json(error_response))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::BadRequestError => StatusCode::BAD_REQUEST,
            ApiError::InvalidCredentials => StatusCode::BAD_REQUEST,
            ApiError::UnAuthorized => StatusCode::UNAUTHORIZED,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            ApiError::TooManyRequest => StatusCode::TOO_MANY_REQUESTS,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::PermissionError => StatusCode::FORBIDDEN,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
        }
    }
}
