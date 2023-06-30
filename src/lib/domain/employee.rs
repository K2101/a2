use super::user::User;
use super::{DomainError, Result};
use crate::config::app_config::AppConfig;
use crate::utils;
use uuid::Uuid;

#[derive(Debug)]
pub struct Employee {
    employee_id: Uuid,
    email: String,
    password: String,
    phone: String,
    role_and_status: User,
}

impl Employee {
    pub fn new(
        app_config: &AppConfig,
        employee_id: Uuid,
        email: String,
        password: String,
        phone: String,
        role: String,
        status: String,
    ) -> Result<Self> {
        let email = email.trim().to_lowercase();
        let password = password.trim().to_string();
        let phone = phone.trim().to_string();
        let role = role.trim().to_uppercase();
        let status = status.trim().to_uppercase();

        if email.is_empty()
            || password.is_empty()
            || phone.is_empty()
            || role.is_empty()
            || status.is_empty()
        {
            return Err(DomainError::EmptyContent("empty content"));
        }

        let password = match utils::hash::hash(app_config, password, None) {
            Ok(pd) => pd,
            Err(err) => {
                println!("domain password error: {:?}", err);
                return Err(DomainError::HashError("hash error".to_string()));
            }
        };
        let role: super::user::Role = role.as_str().try_into()?;
        let status: super::status::Status = status.as_str().try_into()?;
        let role_and_status = User::new(role, status)?;

        Ok(Self {
            employee_id,
            email,
            password,
            phone,
            role_and_status,
        })
    }

    pub fn get_ref(&self) -> (&Uuid, &str, &str, &str, &str, &str) {
        let (role, status) = self.role_and_status.as_str();
        (
            &self.employee_id,
            self.email.as_str(),
            self.password.as_str(),
            self.phone.as_str(),
            role,
            status,
        )
    }
}
