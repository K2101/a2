use crate::data;
use crate::data::database::Database;
use crate::data::key_db::Cache;
use crate::domain;
use actix_web::web;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod user_authorization_mod {
    tonic::include_proto!("user_authorization");
}

use user_authorization_mod::user_authorization_server::UserAuthorization;
use user_authorization_mod::{UserAuthorizationRequest, UserAuthorizationResponse};

#[derive(Debug)]
pub struct UserAuthorizationService {
    pub db: web::Data<Database>,
    pub cache: web::Data<Cache>,
}

impl UserAuthorizationService {
    pub fn new(db: web::Data<Database>, cache: web::Data<Cache>) -> Self {
        Self { db, cache }
    }
}

#[tonic::async_trait]
impl UserAuthorization for UserAuthorizationService {
    async fn user_authorization(
        &self,
        request: Request<UserAuthorizationRequest>,
    ) -> Result<Response<UserAuthorizationResponse>, Status> {
        println!("Got a rpc request");
        let tid = std::thread::current().id();
        println!("rpc {:?}", tid);
        let tname = std::thread::current().name().unwrap().to_owned();
        println!("rpc name{}", tname);

        let session_id = request.into_inner().session;

        if session_id.is_empty() {
            let respond = UserAuthorizationResponse {
                is_authorized: false,
            };
            return Ok(Response::new(respond));
        }

        // query from cache to get session detail
        let (id, email, valid_until) =
            match data::query_key_db::get_session(&self.cache, session_id.as_str()).await {
                Ok(resp) => resp,
                Err(err) => {
                    println!("get session error: {:?}", err);
                    let respond = UserAuthorizationResponse {
                        is_authorized: false,
                    };
                    return Ok(Response::new(respond));
                }
            };

        if id.is_none() || email.is_none() || valid_until.is_none() {
            let respond = UserAuthorizationResponse {
                is_authorized: false,
            };
            return Ok(Response::new(respond));
        }
        let id = id.unwrap();
        let email = email.unwrap();
        let valid_until = valid_until.unwrap();

        let id = id.trim();
        let email = email.trim();
        let valid_until = valid_until.trim();

        if id.is_empty() || email.is_empty() || valid_until.is_empty() {
            let respond = UserAuthorizationResponse {
                is_authorized: false,
            };
            return Ok(Response::new(respond));
        }

        let id = match Uuid::parse_str(id) {
            Ok(id) => id,
            Err(_) => {
                let respond = UserAuthorizationResponse {
                    is_authorized: false,
                };
                return Ok(Response::new(respond));
            }
        };

        let valid_until = match valid_until.parse::<i64>() {
            Ok(vn) => vn,
            Err(_) => {
                let respond = UserAuthorizationResponse {
                    is_authorized: false,
                };
                return Ok(Response::new(respond));
            }
        };

        let now = domain::time::Time::now().into_inner();

        if valid_until < now {
            let respond = UserAuthorizationResponse {
                is_authorized: false,
            };
            return Ok(Response::new(respond));
        }

        // query db
        let result = match data::query::get_customer_credentials(&self.db, email).await {
            Ok(ok) => ok,
            Err(_) => {
                let respond = UserAuthorizationResponse {
                    is_authorized: false,
                };
                return Ok(Response::new(respond));
            }
        };

        let vec_of_row = match result.rows {
            Some(v_r) => v_r,
            None => {
                let respond = UserAuthorizationResponse {
                    is_authorized: false,
                };
                return Ok(Response::new(respond));
            }
        };

        if vec_of_row.len() != 1 {
            println!("get more than one user in rpc");
            let respond = UserAuthorizationResponse {
                is_authorized: false,
            };
            return Ok(Response::new(respond));
        }

        let customer_id = vec_of_row[0].columns[0]
            .as_ref()
            .unwrap()
            .as_uuid()
            .unwrap();

        let status = vec_of_row[0].columns[5]
            .as_ref()
            .unwrap()
            .as_text()
            .unwrap();

        if id != customer_id {
            let respond = UserAuthorizationResponse {
                is_authorized: false,
            };
            return Ok(Response::new(respond));
        }

        let status: domain::status::Status = match status.as_str().try_into() {
            Ok(s) => s,
            Err(_) => {
                let respond = UserAuthorizationResponse {
                    is_authorized: false,
                };
                return Ok(Response::new(respond));
            }
        };

        if status != domain::status::Status::Active {
            let respond = UserAuthorizationResponse {
                is_authorized: false,
            };
            return Ok(Response::new(respond));
        }

        let respond = UserAuthorizationResponse {
            is_authorized: true,
        };
        Ok(Response::new(respond))
    }
}
