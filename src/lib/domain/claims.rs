use super::{DomainError, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // customer_id or employee_id
    id: Uuid,
    session_id: String,
    role: String,
    exp: usize,
}

impl Claims {
    pub fn new(id: Uuid, session_id: &str, role: &str, exp: usize) -> Result<Self> {
        let session_id = session_id.trim().to_string();
        let role = role.trim().to_string();

        if session_id.is_empty() || role.is_empty() {
            return Err(DomainError::InvalidClaims);
        }

        if !role.is_ascii() {
            return Err(DomainError::InvalidClaims);
        }

        Ok(Self {
            id,
            session_id,
            role,
            exp,
        })
    }

    pub fn into_inner(self) -> (Uuid, String, String, usize) {
        (self.id, self.session_id, self.role, self.exp)
    }

    pub fn get_role<'a, T: std::convert::TryFrom<&'a str>>(
        &'a self,
    ) -> std::result::Result<T, crate::service::ServiceError> {
        let role = self.role.as_str().try_into();
        match role {
            Ok(role) => Ok(role),
            Err(_) => Err(crate::service::ServiceError::UnAuthorized),
        }
    }
}
