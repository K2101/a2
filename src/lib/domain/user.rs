use super::{DomainError, Result};
use serde::Deserialize;

#[derive(Debug, PartialEq, PartialOrd, Deserialize)]
pub enum Status {
    Active,
    InActive,
    Terminated,
}

impl From<Status> for &'static str {
    fn from(status: Status) -> Self {
        use Status::*;
        match status {
            Active => "ACTIVE",
            InActive => "INACTIVE",
            Terminated => "TERMINATED",
        }
    }
}

impl From<&Status> for &'static str {
    fn from(status: &Status) -> Self {
        use Status::*;
        match status {
            Active => "ACTIVE",
            InActive => "INACTIVE",
            Terminated => "TERMINATED",
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = DomainError;
    fn try_from(status: &str) -> std::result::Result<Self, Self::Error> {
        let status = status.trim();
        use Status::*;
        match status {
            "ACTIVE" => Ok(Active),
            "INACTIVE" => Ok(InActive),
            "TERMINATED" => Ok(Terminated),
            _ => Err(DomainError::StatusError),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Role {
    PersonalUser,
    CoporateUser,
    ThirdPartyUser,
}

impl From<Role> for &'static str {
    fn from(role: Role) -> Self {
        use Role::*;
        match role {
            PersonalUser => "PERSONAL_USER",
            CoporateUser => "COPORATE_USER",
            ThirdPartyUser => "THIRD_PARTY_USER",
        }
    }
}

impl TryFrom<&str> for Role {
    type Error = DomainError;
    fn try_from(status: &str) -> std::result::Result<Self, Self::Error> {
        let status = status.trim();
        use Role::*;
        match status {
            "PERSONAL_USER" => Ok(PersonalUser),
            "COPORATE_USER" => Ok(CoporateUser),
            "THIRD_PARTY_USER" => Ok(ThirdPartyUser),
            _ => Err(DomainError::RoleError),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct User(Role, Status);
impl User {
    pub fn new(role: Role, status: Status) -> Self {
        Self(role, status)
    }
    pub fn update_status(_internal_user: &InternalUser, user: User, status: Status) -> Self {
        Self(user.0, status)
    }
    pub fn into_inner(self) -> (&'static str, &'static str) {
        (self.0.into(), self.1.into())
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum InternalRole {
    SuperAdminUser,
    AdminUser,
    ManagerUser,
    DeveloperUser,
    AccountUser,
    SupportUser,
}
impl From<InternalRole> for &'static str {
    fn from(internal_role: InternalRole) -> Self {
        use InternalRole::*;
        match internal_role {
            SuperAdminUser => "SUPER_ADMIN_USER",
            AdminUser => "ADMIN_USER",
            ManagerUser => "MANAGER_USER",
            DeveloperUser => "DEVELOPER_USER",
            AccountUser => "ACCOUNT_USER",
            SupportUser => "SUPPORT_USER",
        }
    }
}

impl From<&InternalRole> for &'static str {
    fn from(internal_role: &InternalRole) -> Self {
        use InternalRole::*;
        match internal_role {
            SuperAdminUser => "SUPER_ADMIN_USER",
            AdminUser => "ADMIN_USER",
            ManagerUser => "MANAGER_USER",
            DeveloperUser => "DEVELOPER_USER",
            AccountUser => "ACCOUNT_USER",
            SupportUser => "SUPPORT_USER",
        }
    }
}

impl TryFrom<&str> for InternalRole {
    type Error = DomainError;
    fn try_from(internal_role: &str) -> std::result::Result<Self, Self::Error> {
        let internal_role = internal_role.trim();
        use InternalRole::*;
        match internal_role {
            "SUPER_ADMIN_USER" => Ok(SuperAdminUser),
            "ADMIN_USER" => Ok(AdminUser),
            "MANAGER_USER" => Ok(ManagerUser),
            "DEVELOPER_USER" => Ok(DeveloperUser),
            "ACCOUNT_USER" => Ok(AccountUser),
            "SUPPORT_USER" => Ok(SupportUser),
            _ => Err(DomainError::InternalRoleError),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct InternalUser(InternalRole, Status);

impl InternalUser {
    pub fn new(internal_role: InternalRole, status: Status) -> Result<Self> {
        if internal_role == InternalRole::SuperAdminUser {
            return Err(DomainError::PermissionError(
                "permission error, cannot create a super admin",
            ));
        }
        if internal_role == InternalRole::AdminUser {
            return Err(DomainError::PermissionError(
                "permission error, cannot create an admin user",
            ));
        }
        Ok(Self(internal_role, status))
    }

    pub fn update_internal_role(
        user: &InternalUser,
        update_user: InternalUser,
        role: InternalRole,
    ) -> Result<Self> {
        if user.0 != InternalRole::AdminUser {
            return Err(DomainError::PermissionError(
                "who use this fn must be an admin",
            ));
        }

        Ok(Self(role, update_user.1))
    }

    pub fn update_internal_status(
        user: &InternalUser,
        update_user: InternalUser,
        status: Status,
    ) -> Result<Self> {
        if user.0 != InternalRole::AdminUser {
            return Err(DomainError::PermissionError(
                "who use this fn must be an admin",
            ));
        }

        Ok(Self(update_user.0, status))
    }

    pub fn get_str(&self) -> (&str, &str) {
        let role = &self.0;
        let status = &self.1;
        let role: &str = role.into();
        let status: &str = status.into();
        (role, status)
    }
}
