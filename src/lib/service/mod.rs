pub mod api_action;
pub mod event_action;

use crate::data::DatabaseError;
use crate::domain::AuthError;
use crate::event::EventError;
use flexbuffers::DeserializationError;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("auth error: {0}")]
    AuthError(#[from] AuthError),

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("database error: {0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("not found")]
    NotFound,

    #[error("unauthorized")]
    UnAuthorized,

    #[error("Bad Request error")]
    BadRequestError,

    #[error("forbidden error: {0}")]
    Forbidden(&'static str),

    #[error("permission error: {0}")]
    PermissionError(&'static str),

    #[error("event error: {0}")]
    EventError(#[from] EventError),

    #[error("(concurrency call) database error or event error: {0}")]
    DatabaseOrEventError(String),

    #[error("deserialize error: {0}")]
    DeserializeDataError(#[from] DeserializationError),

    #[error("domain parse error, cannot parse from model to domain")]
    DomainParseError,
    #[error("internal error: {0}")]
    InternalError(&'static str),

    #[error("totp error: {0}")]
    TotpeError(String),
}
