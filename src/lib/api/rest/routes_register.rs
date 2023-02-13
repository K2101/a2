use super::super::ErrorResponseToClient;
use crate::api::rest::internal_user;
use crate::api::rest::user;
use actix_web::error;
use actix_web::{web, HttpResponse};

pub fn auth_routes_scoped(cfg: &mut web::ServiceConfig) {
    let json_config = web::JsonConfig::default().error_handler(|err, _req| {
        let err2 = err.to_string();
        let splited: Vec<&str> = err2.split("at line").collect();
        // for rust type driven conversion first
        // create custom error response
        let error_response = ErrorResponseToClient {
            status: "error".to_string(),
            message: splited[0].trim().to_string(),
        };
        error::InternalError::from_response(err, HttpResponse::BadRequest().json(error_response))
            .into()
    });

    cfg.app_data(json_config)
        .service(user::web_user_login)
        .service(internal_user::web_internal_user_login)
        .service(internal_user::web_internal_refresh_token);
}
