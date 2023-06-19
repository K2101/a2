pub const CONSUMER_GROUP: &'static str = "AUTH";

pub enum Topic {
    // retail customer
    RetailCustomerCreateRequestAfterCheck,

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
            RetailCustomerCreateRequestAfterCheck => "RETAIL_CUSTOMER_CREATE_REQUEST_AFTER_CHECK",
            InternalUserCreateRequest => "INTERNAL_USER_CREATE_REQUEST",
            RetailCustomerApprove => "RETAIL_CUSTOMER_APPROVE",
            EmployeeCreate => "EMPLOYEE_CREATE",
        }
    }
}

impl TryFrom<&str> for Topic {
    type Error = crate::service::ServiceError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "RETAIL_CUSTOMER_CREATE_REQUEST_AFTER_CHECK" => {
                Ok(Self::RetailCustomerCreateRequestAfterCheck)
            }
            "INTERNAL_USER_CREATE_REQUEST" => Ok(Self::InternalUserCreateRequest),
            "RETAIL_CUSTOMER_APPROVE" => Ok(Self::RetailCustomerApprove),
            "EMPLOYEE_CREATE" => Ok(Self::EmployeeCreate),
            _ => Err(Self::Error::InvalidTopic),
        }
    }
}
