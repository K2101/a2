use crate::config::app_config::AppConfig;
use crate::config::app_config::{exit_program, ExitCode};
use crate::data;
use crate::data::database::Database;
use crate::data::key_db::Cache;
use crate::domain;
use crate::domain::user::{Role, Status};
use crate::domain::SEVEN_DAYS;
use crate::event::kafka_client::LoggingConsumer;
use crate::service::{Result, ServiceError};
use crate::utils::backoff::BackoffSetting;
use actix_rt::time;
use actix_web::web;
use flexbuffers::Reader;
use rdkafka::consumer::Consumer;
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::FutureProducer;
use rdkafka::Message;
use serde::Deserialize;

pub async fn user_created(
    app_config: web::Data<AppConfig>,
    consumer: LoggingConsumer,
    database: web::Data<Database>,
    producer: web::Data<FutureProducer>,
    cache: web::Data<Cache>,
) {
    loop {
        match consumer.recv().await {
            Ok(msg) => {
                let result = user_created_action(&app_config, &database, &producer, &msg).await;

                match result {
                    Ok(_) => {
                        println!("consume success");
                    }
                    Err(err) => match err {
                        ServiceError::DatabaseOrEventError(err) => {
                            println!("error: {:?}", err);
                            println!("got retry");
                            let backoff =
                                BackoffSetting::new(3000, 100, SEVEN_DAYS as u64).get_backoff();
                            for duration in &backoff {
                                println!("duration: {:?}", duration);
                                let result =
                                    user_created_action(&app_config, &database, &producer, &msg)
                                        .await;

                                match result {
                                    Ok(()) => {
                                        println!("retry success");
                                        break;
                                    }
                                    Err(err) => {
                                        println!("retry error {:?}", err);
                                        time::sleep(duration).await;
                                    }
                                }
                            }
                        }
                        ServiceError::DeserializeDataError(err) => {
                            println!("error: {:?}", err);
                            return exit_program(ExitCode::ConsumerDeserializeError);
                        }
                        ServiceError::DomainParseError => {
                            println!("error: {:?}", err);
                            return exit_program(ExitCode::InvalidDate);
                        }
                        _ => {
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
                println!("consume error: {}", err);
                return exit_program(ExitCode::ConsumeError);
            }
        }
    }
}

async fn user_created_action(
    app_config: &AppConfig,
    database: &Database,
    _producer: &FutureProducer,
    msg: &BorrowedMessage<'_>,
) -> Result<()> {
    let bytes = BorrowedMessage::payload(msg).unwrap_or_default();
    let reader_bytes = Reader::get_root(bytes).unwrap_or_default();

    match data::model_event::NewRetailCustomer::deserialize(reader_bytes) {
        Ok(nrc) => {
            let data::model_event::NewRetailCustomer {
                customer_id,
                email,
                password,
                phone,
            } = nrc;

            // hard code for new personal user || retail customer
            let role: &str = Role::PersonalUser.into();
            let status: &str = Status::InActive.into();

            let domain_user_result = domain::user_domain::UserDomain::new(
                app_config,
                customer_id,
                email,
                password,
                phone,
                role,
                status,
            );

            match domain_user_result {
                Ok(domain_user) => {
                    let insert_to_db =
                        data::query::insert_customer_credentials(database, domain_user).await;
                    match insert_to_db {
                        Ok(_) => return Ok(()),
                        Err(err) => {
                            return Err(ServiceError::DatabaseOrEventError(err.to_string()))
                        }
                    }
                }
                Err(err) => {
                    println!("error: {:?}", err);
                    return Err(ServiceError::DomainParseError);
                }
            }
        }
        Err(err) => return Err(ServiceError::DeserializeDataError(err)),
    }
}
