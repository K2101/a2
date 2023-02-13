use crate::service::{Result, ServiceError};
use crate::utils;
use totp_rs::{Algorithm, Secret, TOTP};

const ISSUER: &'static str = "Entangle";

pub struct Totp {
    totp: TOTP,
}

impl Totp {
    pub fn new(account_name: Option<String>) -> Result<Self> {
        // 160 bit
        // let secret = utils::rand(20);
        let secret = utils::rand(100);
        let secret_vec = match Secret::Raw(secret.trim().as_bytes().to_vec()).to_bytes() {
            Ok(sr) => sr,
            Err(err) => {
                return Err(ServiceError::TotpeError(format!(
                    "create totp secret error: {:?}",
                    err
                )))
            }
        };

        let account_name = if let Some(an) = account_name {
            an.trim().to_string()
        } else {
            String::default()
        };

        let totp = match TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret_vec,
            Some(ISSUER.to_string()),
            account_name,
        ) {
            Ok(totp) => totp,
            Err(err) => {
                return Err(ServiceError::TotpeError(format!(
                    "create totp instance error: {:?}",
                    err
                )))
            }
        };

        Ok(Self { totp })
    }

    pub fn generate(&self) -> Result<String> {
        let token = match self.totp.generate_current() {
            Ok(token) => token,
            Err(err) => {
                return Err(ServiceError::TotpeError(format!(
                    "generate token error: {:?}",
                    err
                )))
            }
        };

        Ok(token)
    }

    // don't forgot rate limit
    pub fn check(&self, token: &str) -> Result<bool> {
        let is_true = match self.totp.check_current(token) {
            Ok(is_true) => is_true,
            Err(err) => {
                return Err(ServiceError::TotpeError(format!(
                    "check token error: {:?}",
                    err
                )))
            }
        };
        Ok(is_true)
    }

    pub fn get_secret(&self) -> String {
        self.totp.get_secret_base32()
    }
}
