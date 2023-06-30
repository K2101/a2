pub mod claims;
pub mod employee;
pub mod login_mobile;
pub mod login_web;
pub mod session;
pub mod status;
pub mod time;
pub mod user;
pub mod user_domain;

pub const SEVEN_DAYS: i32 = 604800;

pub type Result<T> = std::result::Result<T, DomainError>;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("invalid data: {0}")]
    InvalidData(&'static str),

    #[error("empty content: {0}")]
    EmptyContent(&'static str),

    #[error("status error")]
    StatusError,
    #[error("role error")]
    RoleError,
    #[error("internal role error")]
    InternalRoleError,

    #[error("permission error: {0}")]
    PermissionError(&'static str),

    #[error("get unix nano time error: {0}")]
    GetUnixNanoTimeError(String),

    #[error("invalid claims data")]
    InvalidClaims,

    #[error("encode JWT error: {0}")]
    EncodeJWTError(String),

    #[error("hash error: {0}")]
    HashError(String),

    #[error("convert unix nano to seconds error: {0}")]
    ConvertUnixNanoToSecondsError(String),
}
