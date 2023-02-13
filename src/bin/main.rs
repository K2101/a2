use actix_web::{web, App, HttpServer};
use auth::api::rest::routes_register::auth_routes_scoped;
use auth::api::rpc::rpc_server;
use auth::config::app_config::AppConfig;
use auth::config::config::Config;
use auth::config::kafka_info::CONSUMER_GROUP;
use auth::data::database::Database;
use auth::data::key_db::Cache;
use auth::event::consumer::subscribe_and_consume;
use auth::event::kafka_client::producer_client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::get_config();
    let app_config = AppConfig::get_config();

    let database: Database = Database::connect(
        config.database_url_1.to_owned(),
        config.database_url_2.to_owned(),
        config.database_port,
    )
    .await
    .unwrap()
    .database_setting()
    .await
    .unwrap();

    // let kafka_broker = config.kafka_broker.as_str();
    // let producer = producer_client(kafka_broker).await;
    // let producer = web::Data::new(producer);
    let database = web::Data::new(database);
    let app_config = web::Data::new(app_config);

    let cache = Cache::keydb_create_client_and_connect(&config.key_db).await;
    let cache = web::Data::new(cache);

    // subscribe_and_consume(
    //     kafka_broker,
    //     CONSUMER_GROUP,
    //     config.threads_nums,
    //     app_config.clone(),
    //     database.clone(),
    //     producer.clone(),
    //     cache.clone(),
    // )
    // .await;

    // rpc
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.threads_nums)
        .enable_io()
        .build()
        .expect("cannot create multi thread runtime");

    let rpc_cache = cache.clone();
    let db_rpc = database.clone();
    rt.spawn(async move {
        rpc_server(&config.rpc_server, db_rpc, rpc_cache).await;
    });

    // rest
    HttpServer::new(move || {
        println!("rest");
        App::new()
            // .app_data(producer.clone())
            .app_data(database.clone())
            .app_data(app_config.clone())
            .app_data(cache.clone())
            .service(web::scope("auth/api/v1").configure(auth_routes_scoped))
    })
    .bind((config.url, config.port))?
    .run()
    .await
}
