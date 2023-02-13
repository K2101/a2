use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct WebSession {
//     // customer_id or employee_id
//     pub user_id: Uuid,
//     pub session_id: String,
//     pub created_at: i64,
//     // can be user choice and can use key expire for lnvalid
//     pub valid_until: i64,
//     // 10 second delete
//     pub is_active: bool,
//     pub device_type: String,
//     pub app_name: String,
//     pub ip_address: String,
//     pub location: String,
//     // update every time
//     pub last_active: i64,
// }

#[derive(Debug)]
pub struct WebSessionRef<'a> {
    // customer_id or employee_id
    pub id: &'a str,
    pub session_id_list: &'a str,
}
