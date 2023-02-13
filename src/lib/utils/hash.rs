use crate::config::app_config::AppConfig;
use crate::domain::AuthError;
use crate::service;
use crate::utils::rand::rand;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::{Algorithm, Argon2, Params, Version};

pub fn hash(
    app_config: &AppConfig,
    value: String,
    salt_len: Option<usize>,
) -> service::Result<String> {
    // validation
    // salt_len must less than 30 or 64 in base64

    let value = value.trim().as_bytes();
    let salt_len = if let Some(sl) = salt_len {
        Some(sl)
    } else {
        Some(32)
    };
    let salt = rand(salt_len.unwrap());
    // need to encode salt in base 64 otherwise it will error ????
    let salt_string = match SaltString::b64_encode(salt.as_bytes()) {
        Ok(salt) => salt,
        Err(err) => {
            println!("create salt error: {:?}", err);
            return Err(service::ServiceError::InternalError("create salt error"));
        }
    };

    let argon2 = match Argon2::new_with_secret(
        app_config.argon2_secret.as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    ) {
        Ok(argon2) => argon2,
        Err(err) => {
            println!("error to create argon2: {:?}", err);
            return Err(service::ServiceError::InternalError(
                "error to create argon2",
            ));
        }
    };

    // Hash password to PHC string ($argon2id$v=19$...)
    // $argon2id$v=19$m=4096,t=3,p=1$TH5POnpYaHEuamNmbHlkMEd4Ulc6PHEwfXxFb2hW$a6bYD3U0hlh6zoyapiKjump0zkIXIHAy2jABB7nKYr4
    let value_hashed = match argon2.hash_password(value, &salt_string) {
        Ok(value_hashed) => value_hashed,
        Err(err) => {
            println!("hash error: {:?}", err);
            return Err(AuthError::HashError("hash error".to_string()))?;
        }
    };

    Ok(value_hashed.to_string())
}

pub fn verify_password(app_config: &AppConfig, hash: &str, value: &str) -> service::Result<bool> {
    let parsed_hash = PasswordHash::new(hash.trim()).expect("cannot parse str to PasswordHash");

    let argon2 = match Argon2::new_with_secret(
        app_config.argon2_secret.as_bytes(),
        Algorithm::default(),
        Version::default(),
        Params::default(),
    ) {
        Ok(argon2) => argon2,
        Err(err) => {
            println!("error to create argon2: {:?}", err);
            return Err(service::ServiceError::InternalError(
                "error to create argon2",
            ));
        }
    };

    let result = argon2.verify_password(value.trim().as_bytes(), &parsed_hash);

    match result {
        Ok(_) => return Ok(true),
        Err(err) => {
            println!("verify password error: {:?}", err);
            return Ok(false);
        }
    }
}
