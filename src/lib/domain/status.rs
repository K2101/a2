use crate::domain::DomainError;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Status {
    Active,
    InActive,
    OwnerRequestForDelete,
    Banned,
}

impl From<Status> for &'static str {
    fn from(status: Status) -> Self {
        use Status::*;
        match status {
            Active => "1",
            InActive => "2",
            OwnerRequestForDelete => "3",
            Banned => "4",
        }
    }
}

impl From<&Status> for &'static str {
    fn from(status: &Status) -> Self {
        use Status::*;
        match status {
            Active => "1",
            InActive => "2",
            OwnerRequestForDelete => "3",
            Banned => "4",
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = DomainError;
    fn try_from(status: &str) -> std::result::Result<Self, Self::Error> {
        let status = status.trim();
        use Status::*;
        match status {
            "1" => Ok(Active),
            "2" => Ok(InActive),
            "3" => Ok(OwnerRequestForDelete),
            "4" => Ok(Banned),
            _ => Err(DomainError::StatusError),
        }
    }
}
