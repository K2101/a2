use super::super::{Result, ServiceError};
use crate::config::app_config::{exit_program, AppConfig, ExitCode};
use crate::data;
use crate::data::database::Database;
use crate::domain;
use crate::event::kafka_client::LoggingConsumer;
use crate::utils::backoff::BackoffSetting;
use actix_rt::time;
use actix_web::web;
use flexbuffers::Reader;
use rdkafka::consumer::Consumer;
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::FutureProducer;
use rdkafka::Message;
use serde::Deserialize;

pub async fn employee_create(
    app_config: web::Data<AppConfig>,
    consumer: LoggingConsumer,
    database: web::Data<Database>,
    producer: web::Data<FutureProducer>,
) {
    loop {
        match consumer.recv().await {
            Ok(msg) => {
                let result = employee_create_action(&app_config, &database, &msg).await;
                match result {
                    Ok(_) => println!("consume success"),
                    Err(err) => match err {
                        ServiceError::DatabaseError(err) => {
                            println!("insert employee auth error: {:?}", err);
                            print!("got retry");
                            let backoff = BackoffSetting::default().get_backoff();
                            for duration in &backoff {
                                let result_retry =
                                    employee_create_action(&app_config, &database, &msg).await;
                                match result_retry {
                                    Ok(_) => {
                                        println!("retry success");
                                        break;
                                    }
                                    Err(err) => {
                                        println!("retry error: {:?}", err);
                                        time::sleep(duration).await;
                                    }
                                }
                            }
                        }
                        // ServiceError::EventError(err) => {
                        //     println!("event error: {:?}", err);
                        //     print!("got retry");
                        // }
                        ServiceError::AuthError(err) => {
                            println!("domain error: {:?}", err);
                            return exit_program(ExitCode::InvalidDate);
                        }
                        ServiceError::DeserializeDataError(err) => {
                            println!("deserialize data error: {:?}", err);
                            return exit_program(ExitCode::ConsumerDeserializeError);
                        }
                        err => {
                            println!("internal server error: {:?}", err);
                            return exit_program(ExitCode::InternalServerError);
                        }
                    },
                }

                if let Err(err) = consumer.store_offset_from_message(&msg) {
                    println!("cannot store offset from message: {:?}", err);
                    return exit_program(ExitCode::CannotStoreOffsetFromMessage);
                }
            }
            Err(err) => {
                println!("error to consume: {:?}", err);
                return exit_program(ExitCode::ConsumeError);
            }
        }
    }
}

async fn employee_create_action(
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

            let result = data::query::insert_internal_user_credentials(db, new_emp_domain).await;

            match result {
                Ok(()) => Ok(()),
                Err(err) => Err(ServiceError::DatabaseError(err)),
            }
        }

        Err(err) => Err(ServiceError::DeserializeDataError(err)),
    }
}
