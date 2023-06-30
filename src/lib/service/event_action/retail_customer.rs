use super::super::Result;
use crate::config::app_config::AppConfig;
use crate::data;
use crate::data::database::Database;
use crate::domain;
use crate::domain::status::Status;
use crate::domain::user::Role;
use crate::service::{self, ServiceError};
use flexbuffers::Reader;
use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
use serde::Deserialize;

pub async fn retail_customer_create_after_check(
    app_config: &AppConfig,
    db: &Database,
    msg: &BorrowedMessage<'_>,
) -> Result<()> {
    let bytes = BorrowedMessage::payload(msg).unwrap_or_default();
    let reader_bytes = Reader::get_root(bytes).unwrap_or_default();

    match data::model_event::NewRetailCustomer::deserialize(reader_bytes) {
        Ok(nrc) => {
            let data::model_event::NewRetailCustomer {
                customer_id,
                email,
                password,
                phone,
            } = nrc;

            // hard code for new personal user || retail customer
            let role: &str = Role::User(domain::user::UserRole::Personal).into();
            let status: &str = Status::InActive.into();

            let domain_user_result = domain::user_domain::UserDomain::new(
                app_config,
                customer_id,
                email,
                password,
                phone,
                role,
                status,
            )?;

            data::query::insert_customer_credentials(db, domain_user_result).await?;

            Ok(())
        }
        Err(err) => return Err(ServiceError::DeserializeDataError(err)),
    }
}

pub async fn retail_customer_approve(db: &Database, msg: &BorrowedMessage<'_>) -> Result<()> {
    let bytes = BorrowedMessage::payload(msg).unwrap_or_default();
    let reader_bytes = Reader::get_root(bytes).unwrap_or_default();

    // more in mobile device
    match data::model_event::ApproveRetailCustomer::deserialize(reader_bytes) {
        Ok(appr_rtc) => {
            let approve_rtc_domain: domain::user_domain::ApproveRetailCustomer =
                appr_rtc.try_into()?;

            // check exist before update
            let (email, status) = approve_rtc_domain.into_inner();
            let is_rt_cus_exist = data::query::check_retail_customer_exist(&email, db).await?;

            if is_rt_cus_exist {
                data::query::retail_customer_approve(&email, status, db).await?;
            } else {
                return Err(ServiceError::DatabaseError(
                    scylla::transport::errors::QueryError::InvalidMessage(
                        "this retail_customer is not yet inserted".to_string(),
                    ),
                ));
            }

            Ok(())
        }
        Err(err) => {
            return Err(service::ServiceError::DeserializeDataError(err));
        }
    }
}
