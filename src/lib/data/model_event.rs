use crate::domain;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRetailCustomer {
    pub customer_id: Uuid,
    pub email: String,
    pub password: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct ApproveRetailCustomer {
    pub email: String,
}

impl TryFrom<ApproveRetailCustomer> for domain::user_domain::ApproveRetailCustomer {
    type Error = domain::DomainError;

    fn try_from(appr_rtc: ApproveRetailCustomer) -> Result<Self, Self::Error> {
        domain::user_domain::ApproveRetailCustomer::new(appr_rtc.email)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InternalUser {
    pub role: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct EmployeeCreate {
    pub employee_id: Uuid,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub role_and_status: InternalUser,
}
