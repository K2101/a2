pub mod internal;
pub mod retail_customer;
pub mod user_created;

use super::consumer::retail_customer::retail_customer_approve_fn;
use super::consumer::user_created::user_created_fn;
use crate::config::app_config::AppConfig;
use crate::data::database::Database;
use crate::data::key_db::Cache;
use actix_web::web;
use rdkafka::producer::FutureProducer;

pub async fn subscribe_and_consume(
    broker: &str,
    group_id: &str,
    threads: usize,
    app_config: web::Data<AppConfig>,
    database: web::Data<Database>,
    producer: web::Data<FutureProducer>,
    cache: web::Data<Cache>,
) {
    println!("subscribe_and_consume");
    user_created_fn(
        broker,
        group_id,
        threads,
        app_config.clone(),
        database.clone(),
        producer.clone(),
        cache.clone(),
    )
    .await;

    retail_customer_approve_fn(broker, group_id, threads, database.clone()).await;
    internal::employee_create_fn(
        broker,
        group_id,
        threads,
        app_config.clone(),
        database.clone(),
        producer.clone(),
    )
    .await;
}

pub fn print_threads_detail(num: u16) {
    let tid = std::thread::current().id();
    println!("ab{}{:?}", num, tid);
    let tname = std::thread::current().name().unwrap().to_owned();
    println!("ab{}name{}", num, tname);
}
