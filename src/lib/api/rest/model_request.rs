use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
    pub valid_in_day: u32,
    pub device_id: String,
    pub device_type: String,
    pub app_name: String,
    pub ip_address: String,
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshToken {
    pub ip_address: String,
    pub location: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Totp {
    pub token: String,
    pub secret: String,
}
