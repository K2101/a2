use super::super::{Result, ServiceError};
use crate::api;
use crate::api::rest::model_request::UserLogin;
use crate::api::rest::model_response::UserLoginResponse;
use crate::config::app_config::AppConfig;
use crate::data;
use crate::data::database::Database;
use crate::data::key_db::Cache;
use crate::data::query;
use crate::domain;
use crate::domain::claims::Claims;
use crate::domain::session::{ONE_DAY_IN_NANO, ONE_MINUTES_IN_NANO};
use crate::utils;
use crate::utils::{encode_jwt, rand, verify_password};
use actix_web::cookie::Cookie;
use actix_web::HttpRequest;
use base64::{engine::general_purpose, Engine as _};

pub async fn web_user_login<'c>(
    req: UserLogin,
    database: &Database,
    app_config: &AppConfig,
    cache: &Cache,
) -> Result<(UserLoginResponse, Cookie<'c>)> {
    let get_from_db = query::get_customer_credentials(database, req.email.trim())
        .await?
        .rows;

    if get_from_db.is_none() || get_from_db.as_ref().unwrap().is_empty() {
        return Err(ServiceError::InvalidCredentials);
    }
    let vec_of_row = get_from_db.unwrap();

    let (email, customer_id, password, phone, role, status) =
        utils::db_query_result_helper::get_user_credentials(&vec_of_row);

    let match_password = verify_password(app_config, password, req.password.as_str())?;

    if !match_password {
        return Err(ServiceError::InvalidCredentials);
    }

    let status: domain::user::Status = status.try_into()?;
    if status != domain::user::Status::Active {
        return Err(ServiceError::Forbidden("this user is not active"));
    }
    let session_id = rand(100);
    let session_id = general_purpose::STANDARD.encode(session_id.as_bytes());

    let now = domain::time::Time::now().into_inner();
    // jwt valid for ten minutes
    // why it valid for eleven minutes ??????
    let jwt_valid_until = (ONE_MINUTES_IN_NANO * 9) + now as usize;
    let jwt_valid_until = match domain::time::Time::to_utc_seconds(jwt_valid_until as i64) {
        Ok(time_sec) => time_sec as usize,
        Err(err) => {
            println!("error to convert to unix seconds: {:?}", err);
            return Err(ServiceError::InternalServerError(
                "error to convert to unix seconds",
            ));
        }
    };

    let claims = Claims::new(customer_id, &session_id, role, jwt_valid_until)?;
    let jwt = encode_jwt(app_config, &claims);
    let jwt = match jwt {
        Ok(token) => token,
        Err(err) => {
            println!("error to encode jwt: {:?}", err);
            return Err(ServiceError::InternalServerError("error to encode jwt"));
        }
    };

    let valid_until = if req.valid_in_day > 90 {
        // valid in 200 year
        let max_of_i64: usize = 9223372036854775807;
        max_of_i64
    } else {
        let valid_in_day = ((req.valid_in_day as usize * ONE_DAY_IN_NANO) + now as usize) as usize;
        valid_in_day
    };

    let web_session = domain::session::WebSession::new(
        customer_id,
        req.email,
        session_id.as_str(),
        valid_until,
        req.device_id,
        req.device_type,
        req.app_name,
        req.ip_address,
        req.location,
        now,
    );

    // insert to keyDB
    let result = data::query_key_db::insert_session_to_cache(cache, web_session).await;
    match result {
        Ok(_) => {
            // new response
            // HttpResponse::Ok()
            // .cookie(cookie)
            // .json(json!({"status": "success", "token": token}))

            let cookie = Cookie::build("session_id", session_id)
                .domain("localhost")
                .path("/")
                .secure(true)
                .http_only(true)
                .same_site(actix_web::cookie::SameSite::Strict)
                .finish();
            Ok((UserLoginResponse { jwt }, cookie))
        }
        Err(err) => {
            println!("error to insert sessions to cache: {:?}", err);
            Err(err)
        }
    }
}

pub async fn web_refresh_token(
    req_info: HttpRequest,
    req: api::rest::model_request::RefreshToken,
    database: &Database,
    app_config: &AppConfig,
    cache: &Cache,
) -> Result<()> {
    let gg = utils::refresh_token::web_refresh_token_helper(
        req_info, req, database, app_config, cache, false,
    )
    .await;
    println!("gg :{:?}", gg);
    Ok(())
}
