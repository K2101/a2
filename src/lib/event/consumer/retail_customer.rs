use crate::config::app_config::AppConfig;
use crate::config::kafka_info::Topic;
use crate::data::database::Database;
use crate::data::key_db::Cache;
use crate::event::kafka_client::consumer_client;
use crate::event::kafka_client::LoggingConsumer;
use crate::service;
use actix_rt::Arbiter;
use actix_web::web;
use rdkafka::consumer::Consumer;
use rdkafka::producer::FutureProducer;

pub async fn retail_customer_approve_fn(
    broker: &str,
    group_id: &str,
    threads: usize,
    database: web::Data<Database>,
) {
    for _ in 0..threads {
        let consumer = consumer_client(broker, group_id).await;
        let database = database.clone();

        let ab = Arbiter::new();
        ab.spawn(async move {
            retail_customer_approve(consumer, database).await;
        });
    }
}

async fn retail_customer_approve(consumer: LoggingConsumer, database: web::Data<Database>) {
    let topic = [Topic::RetailCustomerApprove.get_str()];
    consumer
        .subscribe(&topic)
        .expect("Cannot subscribe for 'RetailCustomerApprove'");
    println!("Subscribe to: {:?}", &topic);
    service::event_action::retail_customer::retail_customer_approve(&consumer, &database).await;
}
