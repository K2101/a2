use super::status::Status;
use super::user::{Role, User};
use super::{DomainError, Result};
use crate::config::app_config::AppConfig;
use crate::utils::hash;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserDomain {
    customer_id: Uuid,
    email: String,
    password: String,
    phone: String,
    user: User,
}

impl UserDomain {
    pub fn new(
        app_config: &AppConfig,
        customer_id: Uuid,
        email: String,
        password: String,
        phone: String,
        role: &str,
        status: &str,
    ) -> Result<Self> {
        let email = email.trim().to_lowercase();
        let password = password.trim().to_string();
        let phone = phone.trim().to_string();
        let role = role.trim();
        let status = status.trim();

        let role: Role = role.try_into()?;
        let status: Status = status.try_into()?;
        let user = User::new(role, status)?;

        let hashed = match hash(app_config, password, None) {
            Ok(hashed) => hashed,
            Err(err) => {
                println!("domain hash error: {:?}", err);
                return Err(DomainError::HashError("hash error".to_string()));
            }
        };

        Ok(Self {
            customer_id,
            email,
            password: hashed,
            phone,
            user,
        })
    }

    pub fn into_inner(self) -> (Uuid, String, String, String, &'static str, &'static str) {
        let (role, status) = self.user.into_inner();
        let role: &str = role.into();
        let status: &str = status.into();
        (
            self.customer_id,
            self.email,
            self.password,
            self.phone,
            role,
            status,
        )
    }
}

#[derive(Debug)]
pub struct InternalUserDomain {
    email: String,
    password: String,
    internal_user: User,
}

#[derive(Debug)]
pub struct ApproveRetailCustomer {
    email: String,
    status: Status,
}

impl ApproveRetailCustomer {
    pub fn new(email: String) -> Result<Self> {
        let email = email.trim().to_lowercase();
        // more validation
        if email.is_empty() {
            return Err(DomainError::EmptyContent("email is empty"));
        }
        let status = Status::Active;

        Ok(Self { email, status })
    }

    pub fn into_inner(self) -> (String, &'static str) {
        let status: &str = self.status.into();
        (self.email, status)
    }
}
