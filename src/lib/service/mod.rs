pub mod api_action;
pub mod event_action;

use crate::domain::DomainError;
use flexbuffers::DeserializationError;
use rdkafka::error::KafkaError;
use redis::RedisError;
use scylla::transport::errors::QueryError;

pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("domain error: {0}")]
    DomainError(#[from] DomainError),

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("invalid data: {0}")]
    InvalidData(&'static str),

    #[error("database error: {0}")]
    DatabaseError(#[from] QueryError),

    #[error("event error: {0}")]
    EventError(#[from] KafkaError),

    #[error("not found")]
    NotFound,

    #[error("unauthorized")]
    UnAuthorized,

    #[error("bad Request: {0}")]
    BadRequest(&'static str),

    #[error("forbidden error: {0}")]
    Forbidden(&'static str),

    #[error("permission error: {0}")]
    PermissionError(&'static str),

    #[error("deserialize error: {0}")]
    DeserializeDataError(#[from] DeserializationError),

    #[error("domain parse error, cannot parse from model to domain")]
    DomainParseError,

    #[error("internal server error: {0}")]
    InternalServerError(&'static str),

    #[error("invalid topic")]
    InvalidTopic,

    #[error("cache error: {0}")]
    CacheError(#[from] RedisError),
}
