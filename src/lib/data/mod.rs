pub mod database;
pub mod key_db;
pub mod model_database;
pub mod model_event;
pub mod model_key_db;
pub mod query;
pub mod query_key_db;

use scylla::transport::errors;

pub type Result<T> = std::result::Result<T, DatabaseError>;

// BadKeyspaceName	Invalid keyspace name given to Session::use_keyspace()
// BadQuery	Error caused by caller creating an invalid query
// DbError	An error sent from the database in response to a query as described in the specification\
// NewSessionError	Error that occurred during session creation
// OperationType	Type of the operation rejected by rate limiting
// QueryError	Error that occurred during query execution
// WriteType	Type of write operation requested
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("bad keyspace name: {0}")]
    BadKeyspaceName(#[from] errors::BadKeyspaceName),

    #[error("bad query error: {0}")]
    BadQueryError(#[from] errors::BadQuery),

    #[error("db error: {0}")]
    DbError(#[from] errors::DbError),

    #[error("new session error: {0}")]
    NewSessionError(#[from] errors::NewSessionError),

    #[error("operation type error: {0}")]
    OperationType(String),

    #[error("query error: {0}")]
    QueryError(#[from] errors::QueryError),

    #[error("write type error: {0}")]
    WriteType(String),

    #[error("redis error: {0}")]
    RedisError(#[from] redis::RedisError),
}
