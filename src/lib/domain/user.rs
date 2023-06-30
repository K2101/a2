use super::status::Status;
use super::{DomainError, Result};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserContext {
    pub id: Uuid,
    pub session_id: String,
    pub role: Role,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum UserRole {
    Personal,
    Coporate,
    ThirdParty,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum InternalRole {
    SuperAdmin,
    Admin,
    Manager,
    Developer,
    Account,
    Support,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Role {
    User(UserRole),
    Internal(InternalRole),
}
impl From<&Role> for &'static str {
    fn from(role: &Role) -> Self {
        match role {
            Role::User(r) => match r {
                UserRole::Personal => "PERSONAL",
                UserRole::Coporate => "COPORATE",
                UserRole::ThirdParty => "THIRD_PARTY",
            },
            Role::Internal(ir) => match ir {
                InternalRole::SuperAdmin => "SUPER_ADMIN",
                InternalRole::Admin => "ADMIN",
                InternalRole::Manager => "MANAGER",
                InternalRole::Developer => "DEVELOPER",
                InternalRole::Account => "ACCOUNT",
                InternalRole::Support => "SUPPORT",
            },
        }
    }
}

impl From<Role> for &'static str {
    fn from(role: Role) -> Self {
        match role {
            Role::User(r) => match r {
                UserRole::Personal => "PERSONAL",
                UserRole::Coporate => "COPORATE",
                UserRole::ThirdParty => "THIRD_PARTY",
            },
            Role::Internal(ir) => match ir {
                InternalRole::SuperAdmin => "SUPER_ADMIN",
                InternalRole::Admin => "ADMIN",
                InternalRole::Manager => "MANAGER",
                InternalRole::Developer => "DEVELOPER",
                InternalRole::Account => "ACCOUNT",
                InternalRole::Support => "SUPPORT",
            },
        }
    }
}

impl TryFrom<&str> for Role {
    type Error = DomainError;
    fn try_from(role: &str) -> std::result::Result<Self, Self::Error> {
        use InternalRole::*;
        use UserRole::*;
        match role {
            "PERSONAL" => Ok(Self::User(Personal)),
            "COPORATE" => Ok(Self::User(Coporate)),
            "THIRD_PARTY" => Ok(Self::User(ThirdParty)),

            "SUPER_ADMIN" => Ok(Self::Internal(SuperAdmin)),
            "ADMIN" => Ok(Self::Internal(Admin)),
            "MANAGER" => Ok(Self::Internal(Manager)),
            "DEVELOPER" => Ok(Self::Internal(Developer)),
            "ACCOUNT" => Ok(Self::Internal(Account)),
            "SUPPORT" => Ok(Self::Internal(Support)),
            _ => Err(DomainError::RoleError),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct User(Role, Status);
impl User {
    pub fn new(role: Role, status: Status) -> Result<Self> {
        if role == Role::Internal(InternalRole::SuperAdmin) {
            return Err(DomainError::PermissionError(
                "permission error, cannot create a super admin",
            ));
        }
        if role == Role::Internal(InternalRole::Admin) {
            return Err(DomainError::PermissionError(
                "permission error, cannot create an admin user",
            ));
        }
        Ok(Self(role, status))
    }

    pub fn into_inner(self) -> (Role, Status) {
        (self.0, self.1)
    }

    pub fn update_role(user: User, new_role: Role) -> Result<Self> {
        if user.0 != Role::Internal(InternalRole::Admin) {
            return Err(DomainError::PermissionError(
                "permission error, only admin can update role",
            ));
        }
        let user = Self(new_role, user.1);
        Ok(user)
    }

    pub fn update_status(user: User, status: Status) -> Self {
        Self(user.0, status)
    }
    pub fn as_ref(&self) -> (&Role, &Status) {
        (&self.0, &self.1)
    }

    pub fn as_str(&self) -> (&str, &str) {
        let role = &self.0;
        let role_str: &str = role.into();
        let status = &self.1;
        let status_str: &str = status.into();
        (role_str, status_str)
    }
}
