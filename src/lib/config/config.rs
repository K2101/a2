use std::env;
use std::thread;

#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub port: u16,
    pub database_url_1: String,
    pub database_url_2: String,
    pub database_port: u16,
    pub kafka_broker: String,
    pub threads_nums: usize,
    pub key_db: String,
    pub rpc_server: String,
}

impl Config {
    pub fn get_config() -> Self {
        match env::var("ENVIRONMENT") {
            Ok(envi) if { envi.as_str() == "PRODUCTION" } => return Config::get_production_env(),
            _ => return Config::get_dev_env(),
        };
    }

    fn get_dev_env() -> Self {
        let url = "localhost".to_string();
        let port = 8004;
        let database_url_2 = "127.0.0.1".to_string();
        let database_url_1 = "localhost".to_string();
        let database_port = 9042;
        let kafka_broker = "127.0.0.1:9092".to_string();
        let threads_nums = get_threads_nums();

        // // temp
        // let database_url_1 =
        //     "node-0.aws_ap_southeast_1.95edbe421439ffc84b16.clusters.scylla.cloud".to_string();
        // let database_url_2 =
        //     "node-1.aws_ap_southeast_1.95edbe421439ffc84b16.clusters.scylla.cloud".to_string();

        let key_db = "redis://127.0.0.1:6379".to_string();
        let rpc_server = "127.0.0.1:8005".to_string();

        Self {
            url,
            port,
            database_url_1,
            database_url_2,
            database_port,
            kafka_broker,
            threads_nums,
            key_db,
            rpc_server,
        }
    }
    fn get_production_env() -> Self {
        let url = env::var("URL").expect("URL env is not set");
        let port = env::var("PORT")
            .expect("PORT env is not set")
            .parse::<u16>()
            .expect("cannot parse PORT to u16");

        let database_url_1 = env::var("DATABASE_URL_1").expect("DATABASE_URL_1 env is not set");
        let database_url_2 = env::var("DATABASE_URL_2").expect("DATABASE_URL_2 env is not set");
        let database_port = env::var("DATABASE_PORT")
            .expect("DATABASE_PORT env is not set")
            .parse::<u16>()
            .expect("cannot parse DATABASE_PORT to u16");

        let kafka_broker = env::var("KAFKA_BROKER").expect("KAFKA_BROKER env is not set");
        let threads_nums = get_threads_nums();
        let key_db = env::var("KEY_DB_URL").expect("KEY_DB_URL env is not set");
        let rpc_server = env::var("RPC_SERVER").expect("RPC_SERVER env is not set");

        Self {
            url,
            port,
            database_url_1,
            database_url_2,
            database_port,
            kafka_broker,
            threads_nums,
            key_db,
            rpc_server,
        }
    }
}

fn get_threads_nums() -> usize {
    thread::available_parallelism()
        .expect("Cannot get number of threads")
        .get()
}

pub fn get_process_id() -> u32 {
    std::process::id()
}
