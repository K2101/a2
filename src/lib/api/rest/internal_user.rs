use super::super::{Result, SuccessResponseToClient};
use super::model_response::UserLoginResponse;
use crate::api::rest::model_request;
use crate::config::app_config::AppConfig;
use crate::data::database::Database;
use crate::data::key_db::Cache;
use crate::service;
use crate::utils::authorized_extractor;
use actix_web::http::StatusCode;
use actix_web::{get, post, HttpRequest, HttpResponse};
use actix_web::{web, HttpResponseBuilder};

#[post("/web_internal_user_login")]
pub async fn web_internal_user_login(
    req: web::Json<model_request::UserLogin>,
    database: web::Data<Database>,
    app_config: web::Data<AppConfig>,
    cache: web::Data<Cache>,
) -> Result<HttpResponse> {
    let (result, cookie) = service::api_action::internal_user::web_internal_user_login(
        req.into_inner(),
        &database,
        &app_config,
        &cache,
    )
    .await?;

    let res = HttpResponseBuilder::new(StatusCode::OK)
        .cookie(cookie)
        .body(result.jwt);

    Ok(res)
}

#[post("/web_internal_refresh_token")]
pub async fn web_internal_refresh_token(
    req_info: HttpRequest,
    req: web::Json<model_request::RefreshToken>,
    database: web::Data<Database>,
    app_config: web::Data<AppConfig>,
    cache: web::Data<Cache>,
) -> Result<web::Json<()>> {
    let result = service::api_action::internal_user::web_internal_refresh_token(
        req_info,
        req.into_inner(),
        &database,
        &app_config,
        &cache,
    )
    .await?;
    let respond = SuccessResponseToClient::new(result);
    Ok(web::Json(()))
}
