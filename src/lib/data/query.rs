use crate::data::database::Database;
use crate::domain;
use crate::service::Result;
use scylla::QueryResult;

pub async fn get_customer_credentials(database: &Database, email: &str) -> Result<QueryResult> {
    let session = database.get_session();
    let prepare = database.get_prepare();

    let result = session
        .execute(&prepare.set_get_user_credentials, (email,))
        .await?;

    Ok(result)
}

pub async fn get_internal_user_credentials(
    database: &Database,
    email: &str,
) -> Result<QueryResult> {
    let session = database.get_session();
    let prepare = database.get_prepare();

    let result = session
        .execute(&prepare.set_get_internal_user_credentials, (email,))
        .await?;

    Ok(result)
}

pub async fn insert_customer_credentials(
    database: &Database,
    domain_user: domain::user_domain::UserDomain,
) -> Result<QueryResult> {
    let session = database.get_session();
    let prepare = database.get_prepare();

    let (customer_id, email, password, phone, role, status) = domain_user.into_inner();

    let result = session
        .execute(
            &prepare.insert_user_domain,
            (customer_id, email, password, phone, role, status),
        )
        .await?;

    Ok(result)
}

pub async fn retail_customer_approve(
    approve_rtc: domain::user_domain::ApproveRetailCustomer,
    database: &Database,
) -> Result<()> {
    let session = database.get_session();
    let (email, active_status) = approve_rtc.into_inner();
    session
        .query(
            "UPDATE web_user SET status = ? WHERE email = ?",
            (active_status, email),
        )
        .await?;
    Ok(())
}

pub async fn insert_internal_user_credentials(
    database: &Database,
    domain_user: domain::employee::Employee,
) -> Result<()> {
    let session = database.get_session();
    let prepare = database.get_prepare();

    let (employee_id, email, password, phone, role, status) = domain_user.get_ref();

    let values = (employee_id, email, password, phone, role, status);

    session
        .execute(&prepare.insert_internal_user_domain, values)
        .await?;

    Ok(())
}
