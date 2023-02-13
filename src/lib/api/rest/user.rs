use super::super::{Result, SuccessResponseToClient};
use super::model_response::UserLoginResponse;
use crate::api::rest::model_request;
use crate::config::app_config::AppConfig;
use crate::data::database::Database;
use crate::data::key_db::Cache;
use crate::service;
use actix_web::http::StatusCode;
use actix_web::{get, post};
use actix_web::{web, HttpRequest, HttpResponse, HttpResponseBuilder};

#[post("/web_user_login")]
pub async fn web_user_login(
    req: web::Json<model_request::UserLogin>,
    database: web::Data<Database>,
    app_config: web::Data<AppConfig>,
    cache: web::Data<Cache>,
) -> Result<HttpResponse> {
    let (result, cookie) =
        service::api_action::user::web_user_login(req.into_inner(), &database, &app_config, &cache)
            .await?;
    // let respond = SuccessResponseToClient::new(result);
    // Ok(web::Json(respond))

    let res = HttpResponseBuilder::new(StatusCode::OK)
        .cookie(cookie)
        .body(result.jwt);

    // Ok(res.set_body(respond))
    Ok(res)
}

#[post("/web_refresh_token")]
pub async fn web_refresh_token(
    req_info: HttpRequest,
    req: web::Json<model_request::RefreshToken>,
    database: web::Data<Database>,
    app_config: web::Data<AppConfig>,
    cache: web::Data<Cache>,
) -> Result<web::Json<SuccessResponseToClient<UserLoginResponse>>> {
    let result = service::api_action::user::web_refresh_token(
        req_info,
        req.into_inner(),
        &database,
        &app_config,
        &cache,
    )
    .await?;
    let result = UserLoginResponse {
        jwt: "asd".to_string(),
    };
    let respond = SuccessResponseToClient::new(result);
    Ok(web::Json(respond))
}

#[post("/totp")]
pub async fn totp(
    req_info: HttpRequest,
    req: web::Json<model_request::Totp>,
    app_config: web::Data<AppConfig>,
    database: web::Data<Database>,
) -> Result<web::Json<SuccessResponseToClient<bool>>> {
    let respond = SuccessResponseToClient::new(true);
    Ok(web::Json(respond))
}
