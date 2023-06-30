use super::jwt;
use crate::api::ApiError;
use crate::config::app_config::AppConfig;
use crate::domain;
use crate::domain::user::Role;
use actix_web::web;
use actix_web::HttpMessage;
use actix_web::{dev::ServiceRequest, Error};

pub async fn authorization(req: &mut ServiceRequest) -> Result<Vec<String>, Error> {
    let unauthorized: Error = ApiError::UnAuthorized.into();
    let app_config: &AppConfig = req.app_data::<web::Data<AppConfig>>().unwrap();

    let jwt = match jwt::extract_jwt(req.request()) {
        Ok(jwt) => jwt,
        Err(_) => return Err(unauthorized),
    };

    let claims = match jwt::decode_jwt_take_secret(&app_config.jwt_secret, jwt) {
        Ok(c) => c,
        Err(_) => return Err(unauthorized),
    };

    let (id, session_id, role, _) = claims.into_inner();
    let role_enum: Role = match role.as_str().try_into() {
        Ok(r) => r,
        Err(_) => return Err(unauthorized),
    };

    let user = domain::user::UserContext {
        id,
        session_id,
        role: role_enum,
    };

    let mut ex = req.extensions_mut();
    ex.insert(user);

    Ok(vec![role])
}
