use super::super::{Result, ServiceError};
use crate::config::app_config::AppConfig;
use crate::data;
use crate::data::database::Database;
use crate::domain;
use crate::event::kafka_client::LoggingConsumer;
use flexbuffers::Reader;
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::FutureProducer;
use rdkafka::Message;
use serde::Deserialize;

pub async fn employee_create(
    app_config: &AppConfig,
    db: &Database,
    msg: &BorrowedMessage<'_>,
) -> Result<()> {
    let bytes = BorrowedMessage::payload(msg).unwrap_or_default();
    let reader_bytes = Reader::get_root(bytes).unwrap_or_default();

    match data::model_event::EmployeeCreate::deserialize(reader_bytes) {
        Ok(new_emp) => {
            let new_emp_domain = domain::employee::Employee::new(
                app_config,
                new_emp.employee_id,
                new_emp.email,
                new_emp.password,
                new_emp.phone,
                new_emp.role_and_status.role,
                new_emp.role_and_status.status,
            )?;

            // check if email or employee already exist ??
            // todo create table like unique check lightweight transaction
            // like retail_customer unique email checking before produce employcreate
            data::query::insert_internal_user_credentials(db, new_emp_domain).await?;
            Ok(())
        }

        Err(err) => Err(ServiceError::DeserializeDataError(err)),
    }
}
