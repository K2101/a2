use redis::aio::MultiplexedConnection;
use redis::Client;

#[derive(Debug)]
pub struct Cache {
    conn: MultiplexedConnection,
}

impl Cache {
    pub async fn keydb_create_client_and_connect(url: &str) -> Self {
        // tls rediss://127.0.0.1:6379
        let client = Client::open(url).expect("cannot create keydb client");

        let conn = client
            .get_multiplexed_tokio_connection()
            .await
            .expect("cannot connect to key_db");

        Self { conn }
    }

    pub fn get_conn(&self) -> MultiplexedConnection {
        self.conn.clone()
    }

    // pub fn get_client(&self) -> &Client {
    //     &self.client
    // }
}

// not work in ts_request service, it still hard block thread anyway
// async fn key_sub(cache: &Cache) {
//     println!("init");
//     let mut sync_con = cache.get_client().get_connection().unwrap();

//     let mut sub = sync_con.as_pubsub();

//     println!("init2");
//     sub.subscribe("some_chan").unwrap();
//     println!("block");

//     let msg = sub.get_message().unwrap();
//     let payload: String = msg.get_payload().unwrap();
//     println!("channel '{}': {}", msg.get_channel_name(), payload);

//     println!("final");
// }
