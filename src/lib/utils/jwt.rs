use crate::config::app_config::AppConfig;
use crate::domain::claims::Claims;
use crate::domain::DomainError;
use crate::service::ServiceError;
use actix_web::http::header;
use actix_web::HttpRequest;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub fn encode_jwt(app_config: &AppConfig, claims: &Claims) -> Result<String, DomainError> {
    // The default algorithm is HS256, which uses a shared secret.
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(app_config.jwt_secret.as_ref()),
    );

    match token {
        Ok(token) => Ok(token),
        Err(err) => Err(DomainError::EncodeJWTError(err.to_string())),
    }
}

pub fn decode_jwt(app_config: &AppConfig, token: &str) -> Result<Claims, String> {
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(app_config.jwt_secret.as_ref()),
        &Validation::default(),
    );

    match token {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => Err(err.to_string()),
    }
}

pub fn decode_jwt_take_secret(secret: &str, token: &str) -> Result<Claims, ServiceError> {
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    match token {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => Err(ServiceError::UnAuthorized),
    }
}

pub fn extract_jwt(req_info: &HttpRequest) -> Result<&str, ServiceError> {
    let authorization_header = req_info.headers().get(header::AUTHORIZATION);

    if authorization_header.is_none() {
        return Err(ServiceError::UnAuthorized);
    }
    let mut jwt: Vec<&str> = Vec::with_capacity(2);
    jwt = authorization_header
        .unwrap()
        .to_str()
        .unwrap_or_default()
        .trim()
        .split(' ')
        .collect();
    if authorization_header.unwrap().is_empty()
        || jwt.is_empty()
        || jwt.len() > 2
        || jwt[0].is_empty()
        || jwt[1].is_empty()
    {
        return Err(ServiceError::UnAuthorized);
    }

    Ok(jwt[1])
}
