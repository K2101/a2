use crate::config::app_config::AppConfig;
use crate::config::kafka_info::Topic;
use crate::data::database::Database;
use crate::event::consumer::print_threads_detail;
use crate::event::kafka_client::consumer_client;
use crate::event::kafka_client::LoggingConsumer;
use crate::service;
use actix_rt::Arbiter;
use actix_web::web;
use rdkafka::consumer::Consumer;
use rdkafka::producer::FutureProducer;

pub async fn employee_create_fn(
    broker: &str,
    group_id: &str,
    threads: usize,
    app_config: web::Data<AppConfig>,
    database: web::Data<Database>,
    producer: web::Data<FutureProducer>,
) {
    for _ in 0..threads {
        let consumer = consumer_client(broker, group_id).await;
        let app_config = app_config.clone();
        let database = database.clone();
        let producer = producer.clone();

        let ab = Arbiter::new();
        ab.spawn(async move {
            employee_create(app_config, consumer, database, producer).await;
        });
    }
}

async fn employee_create(
    app_config: web::Data<AppConfig>,
    consumer: LoggingConsumer,
    database: web::Data<Database>,
    producer: web::Data<FutureProducer>,
) {
    let topic = [Topic::EmployeeCreate.get_str()];
    consumer
        .subscribe(&topic)
        .expect("Cannot subscribe for 'EmployeeCreate'");
    println!("Subscribe to: {:?}", &topic);
    service::event_action::employee_create(app_config, consumer, database, producer).await;
}
