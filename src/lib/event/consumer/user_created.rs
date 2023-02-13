use crate::config::app_config::AppConfig;
use crate::config::kafka_info::Topic;
use crate::data::database::Database;
use crate::data::key_db::Cache;
use crate::event::consumer::print_threads_detail;
use crate::event::kafka_client::consumer_client;
use crate::event::kafka_client::LoggingConsumer;
use crate::service;
use actix_rt::Arbiter;
use actix_web::web;
use rdkafka::consumer::Consumer;
use rdkafka::producer::FutureProducer;

pub async fn user_created_fn(
    broker: &str,
    group_id: &str,
    threads: usize,
    app_config: web::Data<AppConfig>,
    database: web::Data<Database>,
    producer: web::Data<FutureProducer>,
    cache: web::Data<Cache>,
) {
    for _ in 0..threads {
        let app_config = app_config.clone();
        let consumer = consumer_client(broker, group_id).await;
        let database = database.clone();
        let producer = producer.clone();
        let cache = cache.clone();

        let ab = Arbiter::new();
        ab.spawn(async move {
            print_threads_detail(1);
            user_created(app_config, consumer, database, producer, cache).await;
        });
    }
}

async fn user_created(
    app_config: web::Data<AppConfig>,
    consumer: LoggingConsumer,
    database: web::Data<Database>,
    producer: web::Data<FutureProducer>,
    cache: web::Data<Cache>,
) {
    let topic = [Topic::RetailCustomerCreateRequest.get_str()];
    consumer
        .subscribe(&topic)
        .expect("Cannot subscribe for 'RetailCustomerCreateRequest'");
    println!("Subscribe to: {:?}", &topic);
    service::event_action::user_created(app_config, consumer, database, producer, cache).await;
}
