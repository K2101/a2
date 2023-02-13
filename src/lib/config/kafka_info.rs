pub const CONSUMER_GROUP: &'static str = "AUTH";

pub enum Topic {
    // retail customer
    RetailCustomerCreateRequest,

    // internal user, employee
    InternalUserCreateRequest,
    // admin approve
    RetailCustomerApprove,

    // internal
    EmployeeCreate,
}

impl Topic {
    pub fn get_str(&self) -> &'static str {
        use Topic::*;
        match self {
            RetailCustomerCreateRequest => "RETAIL_CUSTOMER_CREATE_REQUEST",
            InternalUserCreateRequest => "INTERNAL_USER_CREATE_REQUEST",
            RetailCustomerApprove => "RETAIL_CUSTOMER_APPROVE",
            EmployeeCreate => "EMPLOYEE_CREATE",
        }
    }
}
