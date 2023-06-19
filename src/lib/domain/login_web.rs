use super::{DomainError, Result};

#[derive(Debug)]
pub struct WebUser {
    email: String,
    password: String,
    mfa: Option<String>,
}

impl WebUser {
    pub fn new(email: String, password: String, mfa: Option<String>) -> Result<Self> {
        let email = email.trim().to_string();
        let password = password.trim().to_string();
        let mut mfa_mut: Option<String> = Some(String::default());

        if let Some(mfa) = mfa {
            mfa_mut = Some(mfa.trim().to_string());
            if mfa_mut.as_ref().unwrap().is_empty() {
                return Err(DomainError::EmptyContent("mfa is empty"));
            }
        } else {
            mfa_mut = None;
        }

        // more validation
        if email.is_empty() {
            return Err(DomainError::EmptyContent("email is empty"));
        }
        if password.is_empty() {
            return Err(DomainError::EmptyContent("password is empty"));
        }

        Ok(Self {
            email,
            password,
            mfa: mfa_mut,
        })
    }
}

#[derive(Debug)]
pub struct WebInternalUser {
    email: String,
    password: String,
    mfa: String,
}

impl WebInternalUser {
    pub fn new(email: String, password: String, mfa: String) -> Result<Self> {
        let email = email.trim().to_string();
        let password = password.trim().to_string();
        let mfa = mfa.trim().to_string();
        // more validation
        if email.is_empty() {
            return Err(DomainError::EmptyContent("email is empty"));
        }
        if password.is_empty() {
            return Err(DomainError::EmptyContent("password is empty"));
        }

        if mfa.is_empty() {
            return Err(DomainError::EmptyContent("mfa is empty"));
        }

        Ok(Self {
            email,
            password,
            mfa,
        })
    }
}
