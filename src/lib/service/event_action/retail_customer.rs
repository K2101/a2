use super::super::Result;
use crate::config::app_config::{exit_program, ExitCode};
use crate::data;
use crate::data::database::Database;
use crate::domain;
use crate::event::kafka_client::LoggingConsumer;
use crate::service;
use crate::utils::backoff::BackoffSetting;
use actix_rt::time;
use domain::SEVEN_DAYS;
use flexbuffers::Reader;
use rdkafka::consumer::Consumer;
use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
use serde::Deserialize;

pub async fn retail_customer_approve(consumer: &LoggingConsumer, database: &Database) {
    loop {
        match consumer.recv().await {
            Ok(msg) => {
                let result = retail_customer_approve_action(database, &msg).await;

                match result {
                    Ok(_) => println!("consume success"),
                    Err(err) => match err {
                        service::ServiceError::DatabaseError(err) => {
                            println!("error approve update: {:?}", err);
                            println!("got retry");
                            let backoff =
                                BackoffSetting::new(3000, 100, SEVEN_DAYS as u64).get_backoff();
                            for duration in &backoff {
                                let result_retry =
                                    retail_customer_approve_action(database, &msg).await;
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
                        service::ServiceError::AuthError(err) => {
                            println!("domain error: {:?}", err);
                        }
                        _ => exit_program(ExitCode::InternalServerError),
                    },
                }

                if let Err(err) = consumer.store_offset_from_message(&msg) {
                    println!("cannot store offset from message: {:?}", err);
                    return exit_program(ExitCode::CannotStoreOffsetFromMessage);
                }
            }
            Err(err) => {
                println!("error to consume message: {:?}", err);
                exit_program(ExitCode::ConsumeError);
            }
        }
    }
}

async fn retail_customer_approve_action(
    database: &Database,
    msg: &BorrowedMessage<'_>,
) -> Result<()> {
    let bytes = BorrowedMessage::payload(msg).unwrap_or_default();
    let reader_bytes = Reader::get_root(bytes).unwrap_or_default();

    // more in mobile device
    match data::model_event::ApproveRetailCustomer::deserialize(reader_bytes) {
        Ok(appr_rtc) => {
            let approve_rtc_domain: domain::user_domain::ApproveRetailCustomer =
                appr_rtc.try_into()?;
            data::query::retail_customer_approve(approve_rtc_domain, database).await?;
            Ok(())
        }
        Err(err) => {
            return Err(service::ServiceError::DeserializeDataError(err));
        }
    }
}
