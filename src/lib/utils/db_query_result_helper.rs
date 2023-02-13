use scylla::frame::response::result::Row;
use uuid::Uuid;

pub fn get_internal_user_credentials<'a>(
    vec_of_row: &'a Vec<Row>,
) -> (&'a str, Uuid, &'a str, &'a str, &'a str, &'a str) {
    let email = vec_of_row[0].columns[0]
        .as_ref()
        .unwrap()
        .as_text()
        .expect("cannot extract email");
    let employee_id = vec_of_row[0].columns[1]
        .as_ref()
        .unwrap()
        .as_uuid()
        .expect("cannot extract employee_id");
    let password = vec_of_row[0].columns[2]
        .as_ref()
        .unwrap()
        .as_text()
        .expect("cannot extract password");
    let phone = vec_of_row[0].columns[3]
        .as_ref()
        .unwrap()
        .as_text()
        .expect("cannot extract phone");
    let role = vec_of_row[0].columns[4]
        .as_ref()
        .unwrap()
        .as_text()
        .expect("cannot extract role");
    let status = vec_of_row[0].columns[5]
        .as_ref()
        .unwrap()
        .as_text()
        .expect("cannot extract status");

    (email, employee_id, password, phone, role, status)
}

pub fn get_user_credentials<'a>(
    vec_of_row: &'a Vec<Row>,
) -> (&'a str, Uuid, &'a str, &'a str, &'a str, &'a str) {
    let email = vec_of_row[0].columns[0]
        .as_ref()
        .unwrap()
        .as_text()
        .expect("cannot extract email");
    let customer_id = vec_of_row[0].columns[1]
        .as_ref()
        .unwrap()
        .as_uuid()
        .expect("cannot extract customer_id");
    let password = vec_of_row[0].columns[2]
        .as_ref()
        .unwrap()
        .as_text()
        .expect("cannot extract password");
    let phone = vec_of_row[0].columns[3]
        .as_ref()
        .unwrap()
        .as_text()
        .expect("cannot extract phone");
    let role = vec_of_row[0].columns[4]
        .as_ref()
        .unwrap()
        .as_text()
        .expect("cannot extract role");
    let status = vec_of_row[0].columns[5]
        .as_ref()
        .unwrap()
        .as_text()
        .expect("cannot extract status");

    (email, customer_id, password, phone, role, status)
}
