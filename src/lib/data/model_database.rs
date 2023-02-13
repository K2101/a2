use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSessionHistory {
    // customer_id or employee_id
    pub user_id: String,
    pub session_id: String,
    pub created_at: i64,
    pub valid_until: i64,
    pub device_type: String,
    pub app_name: String,
    pub ip_address: String,
    pub location: String,
    pub last_active: i64,
}
