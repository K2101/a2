use crate::config::app_config::{exit_program, AppConfig, ExitCode};
use crate::config::kafka_info::Topic;
use crate::data::database::Database;
use crate::event;
use crate::event::kafka_client::consumer_client;
use crate::service;
use crate::service::{Result, ServiceError};
use crate::utils::backoff::BackoffSetting;
use actix_rt::time;
use actix_rt::Arbiter;
use actix_web::web;
use rdkafka::consumer::Consumer;
use rdkafka::message::{BorrowedMessage, Message};
use rdkafka::producer::FutureProducer;

pub async fn subscribe_and_consume(
    broker: &str,
    group_id: &str,
    threads: usize,
    database: web::Data<Database>,
    app_config: web::Data<AppConfig>,
    producer: web::Data<FutureProducer>,
) {
    for _ in 0..threads {
        let consumer = consumer_client(broker, group_id).await;
        let database = database.clone();
        let app_config = app_config.clone();
        let producer = producer.clone();
        let ab = Arbiter::new();
        ab.spawn(async move {
            let topic = [
                Topic::EmployeeCreate.get_str(),
                Topic::RetailCustomerApprove.get_str(),
                Topic::RetailCustomerCreateRequestAfterCheck.get_str(),
            ];
            consumer.subscribe(&topic).unwrap();
            println!("Subscribe to: {:?}", &topic);

            consuming(consumer, database, app_config, producer).await;
        });
    }
    println!("subscribe_and_consume");
}

async fn consuming(
    consumer: event::kafka_client::LoggingConsumer,
    database: web::Data<Database>,
    app_config: web::Data<AppConfig>,
    producer: web::Data<FutureProducer>,
) {
    loop {
        match consumer.recv().await {
            Ok(msg) => {
                let result = consuming_action(&database, &app_config, &producer, &msg).await;
                match result {
                    Ok(_) => println!("consume success"),
                    Err(err) => match err {
                        ServiceError::DatabaseError(err) => {
                            println!("database error: {:?}", err);
                            println!("got retry");
                            let backoff = BackoffSetting::default().get_backoff();
                            for duration in &backoff {
                                let result_retry =
                                    consuming_action(&database, &app_config, &producer, &msg).await;
                                match result_retry {
                                    Ok(()) => {
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
                        ServiceError::CacheError(err) => {
                            println!("cache error: {:?}", err);
                            println!("got retry");
                            let backoff = BackoffSetting::default().get_backoff();
                            for duration in &backoff {
                                let result_retry =
                                    consuming_action(&database, &app_config, &producer, &msg).await;
                                match result_retry {
                                    Ok(()) => {
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
                        ServiceError::EventError(err) => {
                            println!("event error: {:?}", err);
                            return exit_program(ExitCode::EventError);
                        }
                        ServiceError::DomainError(err) => {
                            println!("domain error: {:?}", err);
                            // return exit_program(ExitCode::InternalServerError);
                            // log
                        }
                        ServiceError::DeserializeDataError(err) => {
                            println!("deserialize data error: {:?}", err);
                            // return exit_program(ExitCode::ConsumerDeserializeError);
                            // just log for prevent hacker ddos
                        }
                        err => {
                            println!("internal server error: {:?}", err);
                            // log
                            // return exit_program(ExitCode::InternalServerError);
                        }
                    },
                }

                if let Err(err) = consumer.store_offset_from_message(&msg) {
                    println!("cannot store offset from message: {:?}", err);
                    return exit_program(ExitCode::CannotStoreOffsetFromMessage);
                }
            }
            Err(err) => {
                println!("consume error: {:?}", err);
            }
        }
    }
}

async fn consuming_action(
    db: &Database,
    app_config: &AppConfig,
    producer: &FutureProducer,
    msg: &BorrowedMessage<'_>,
) -> Result<()> {
    let topic: Topic = msg.topic().try_into().expect("error to unwrap topic");

    match topic {
        Topic::EmployeeCreate => {
            service::event_action::internal::employee_create(app_config, db, msg).await?;
        }
        Topic::RetailCustomerApprove => {
            service::event_action::retail_customer::retail_customer_approve(db, msg).await?;
        }
        Topic::RetailCustomerCreateRequestAfterCheck => {
            service::event_action::retail_customer::retail_customer_create_after_check(
                app_config, db, msg,
            )
            .await?;
        }

        _ => unreachable!(),
    }

    Ok(())
}

fn print_threads_detail(num: u16) {
    let tid = std::thread::current().id();
    println!("ab{}{:?}", num, tid);
    let tname = std::thread::current().name().unwrap().to_owned();
    println!("ab{}name{}", num, tname);
}
