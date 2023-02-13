pub mod user_authorization;

use crate::data::database::Database;
use crate::data::key_db::Cache;
use actix_web::web;
use std::net::SocketAddr;
use tonic::transport::Server;
use user_authorization::user_authorization_mod::user_authorization_server::UserAuthorizationServer;
use user_authorization::UserAuthorizationService;

pub async fn rpc_server(addr: &str, db: web::Data<Database>, cache: web::Data<Cache>) {
    let addr: SocketAddr = addr.parse().expect("cannot parse addr to SocketAddr");
    let user_authorization_service = UserAuthorizationService::new(db, cache);

    let tid = std::thread::current().id();
    println!("mod rpc {:?}", tid);
    let tname = std::thread::current().name().unwrap().to_owned();
    println!("mod rpc name{}", tname);
    Server::builder()
        .add_service(UserAuthorizationServer::new(user_authorization_service))
        .serve(addr)
        .await
        .expect("cannot create gRPC server");

    // println!("rpc listening at: {}", addr);
}
