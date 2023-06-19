use crate::api;
use crate::api::rest::model_response::UserLoginResponse;
use crate::config::app_config::AppConfig;
use crate::data;
use crate::data::database::Database;
use crate::data::key_db::Cache;
use crate::domain;
use crate::domain::session::ONE_MINUTES_IN_NANO;
use crate::service;
use crate::service::Result;
use crate::utils;
use actix_web::HttpRequest;

pub async fn web_refresh_token_helper(
    req_info: HttpRequest,
    req: api::rest::model_request::RefreshToken,
    db: &Database,
    app_config: &AppConfig,
    cache: &Cache,
    for_internal: bool,
) -> Result<UserLoginResponse> {
    println!("http_request: {:?}", req_info);
    let jwt = utils::jwt::extract_jwt(&req_info)?;
    println!("jwt {}", jwt);

    // what if user just push randam jwt from internet ??
    // can we relia only rand(50) session_id ?
    // maybe yes and since cookie is httpOnly
    // also add rate limit to this route
    match utils::jwt::decode_jwt(app_config, jwt) {
        // jwt still valid
        Ok(_) => return Err(service::ServiceError::UnAuthorized),
        Err(_) => (),
    }

    // session_id=QUJHLipwckl9TUJ8YURjVGNMWWk2W0ZPLlJ8fVZOZW8wUio+bTBoemM0Sl5rKDVCJTg=; HttpOnly; SameSite=Strict; Secure; Path=/; Domain=localhost

    let cookie = match req_info.cookie("session_id") {
        Some(cookie) => cookie,
        None => return Err(service::ServiceError::UnAuthorized),
    };
    let session_id = cookie.value();

    let (id, email, valid_until) = data::query_key_db::get_session(cache, session_id).await?;
    if id.is_none() || email.is_none() || valid_until.is_none() {
        return Err(service::ServiceError::UnAuthorized);
    }

    let id = id.unwrap();
    let email = email.unwrap();
    let valid_until = valid_until.unwrap();

    let id = id.trim();
    let valid_until = valid_until.trim();
    let email = email.trim();
    let email_from_cache = email.to_lowercase();

    if id.is_empty() || valid_until.is_empty() || email.is_empty() {
        return Err(service::ServiceError::UnAuthorized);
    }

    let id = match uuid::Uuid::parse_str(id) {
        Ok(id) => id,
        Err(err) => {
            println!("parse str to uuid error: {:?}", err);
            return Err(service::ServiceError::UnAuthorized);
        }
    };

    let valid_until = match valid_until.parse::<i64>() {
        Ok(v) => v,
        Err(err) => {
            println!("parse str to i64 error: {:?}", err);
            return Err(service::ServiceError::UnAuthorized);
        }
    };

    let now = domain::time::Time::now().into_inner();
    if valid_until < now {
        return Err(service::ServiceError::UnAuthorized);
    }

    if for_internal {
        let get_from_db = data::query::get_internal_user_credentials(db, email)
            .await?
            .rows;

        if get_from_db.is_none() || get_from_db.as_ref().unwrap().is_empty() {
            return Err(service::ServiceError::UnAuthorized);
        }
        let vec_of_row = get_from_db.unwrap();

        let (email, employee_id, password, phone, role, status) =
            utils::db_query_result_helper::get_internal_user_credentials(&vec_of_row);

        if id != employee_id {
            return Err(service::ServiceError::UnAuthorized);
        }

        if email_from_cache != email.trim().to_lowercase() {
            return Err(service::ServiceError::UnAuthorized);
        }

        let status: domain::user::Status = status.try_into()?;
        if status != domain::user::Status::Active {
            return Err(service::ServiceError::Forbidden("this user is not active"));
        }

        data::query_key_db::update_refresh_session(
            cache,
            id,
            session_id,
            req.ip_address.trim(),
            req.location.trim(),
            now,
        )
        .await?;

        let jwt_valid_until = (ONE_MINUTES_IN_NANO * 9) + now as usize;
        let jwt_valid_until = match domain::time::Time::to_utc_seconds(jwt_valid_until as i64) {
            Ok(time_sec) => time_sec as usize,
            Err(err) => {
                println!("error to convert to unix seconds: {:?}", err);
                return Err(service::ServiceError::InternalServerError(
                    "error to convert to unix seconds",
                ));
            }
        };

        let claims = domain::claims::Claims::new(id, session_id, role, jwt_valid_until)?;
        let jwt = super::encode_jwt(app_config, &claims)?;

        Ok(UserLoginResponse { jwt })
    } else {
        let get_from_db = data::query::get_customer_credentials(db, email).await?.rows;

        if get_from_db.is_none() || get_from_db.as_ref().unwrap().is_empty() {
            return Err(service::ServiceError::UnAuthorized);
        }
        let vec_of_row = get_from_db.unwrap();

        let (email, customer_id, password, phone, role, status) =
            utils::db_query_result_helper::get_user_credentials(&vec_of_row);

        if id != customer_id {
            return Err(service::ServiceError::UnAuthorized);
        }

        if email_from_cache != email.trim().to_lowercase() {
            return Err(service::ServiceError::UnAuthorized);
        }

        let status: domain::user::Status = status.try_into()?;
        if status != domain::user::Status::Active {
            return Err(service::ServiceError::Forbidden("this user is not active"));
        }

        data::query_key_db::update_refresh_session(
            cache,
            id,
            session_id,
            req.ip_address.trim(),
            req.location.trim(),
            now,
        )
        .await?;

        let jwt_valid_until = (ONE_MINUTES_IN_NANO * 9) + now as usize;
        let jwt_valid_until = match domain::time::Time::to_utc_seconds(jwt_valid_until as i64) {
            Ok(time_sec) => time_sec as usize,
            Err(err) => {
                println!("error to convert to unix seconds: {:?}", err);
                return Err(service::ServiceError::InternalServerError(
                    "error to convert to unix seconds",
                ));
            }
        };

        let claims = domain::claims::Claims::new(id, session_id, role, jwt_valid_until)?;
        let jwt = super::encode_jwt(app_config, &claims)?;

        Ok(UserLoginResponse { jwt })
    }
}
