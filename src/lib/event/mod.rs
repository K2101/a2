pub mod consumer;
pub mod kafka_client;
pub mod producer;

use rdkafka::error::KafkaError;

pub type Result<T> = std::result::Result<T, EventError>;

#[derive(Debug, thiserror::Error)]
pub enum EventError {
    #[error("kafka error: {0}")]
    KafkaError(#[from] KafkaError),
    #[error("serialize error")]
    SerializeError(String),
}
