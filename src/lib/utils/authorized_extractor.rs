// use super::jwt;
// use crate::api;
// use crate::api::ApiError;
// use crate::domain::claims::Claims;
// use crate::domain::user::{InternalRole, Role};
// use crate::service;
// use actix_web::dev::Payload;
// use actix_web::{FromRequest, HttpRequest};
// use std::env;
// use std::future::Future;
// use std::pin::Pin;

// const JWT_DEV: &str = ">tz]6X&q09A4T[1R#1+9zTbS1lc<dyMcU~c&],qyHwt)^9H3vQ*HV5kb[M/nP6/vH$}6rLp.L1$_jWbB$We^47831{]U|[5kG60Y";

// // PersonalUser => "PERSONAL_USER",
// // CoporateUser => "COPORATE_USER",
// // ThirdPartyUser => "THIRD_PARTY_USER",

// pub trait Extractor: Sized {
//     fn new(http_request: HttpRequest, claims: Claims) -> Result<Self, ApiError>;
//     fn into_inner(self) -> (HttpRequest, Claims, Role);
//     fn compare_role(&self, role: Role) -> Result<(), service::ServiceError>;
// }

// #[derive(Debug)]
// pub struct PersonalUser {
//     http_request: HttpRequest,
//     claims: Claims,
//     role: Role,
// }

// impl Extractor for PersonalUser {
//     fn new(http_request: HttpRequest, claims: Claims) -> Result<Self, ApiError> {
//         let role = Role::PersonalUser;
//         Ok(Self {
//             http_request,
//             claims,
//             role,
//         })
//     }

//     fn into_inner(self) -> (HttpRequest, Claims, Role) {
//         (self.http_request, self.claims, self.role)
//     }

//     fn compare_role(&self, role: Role) -> Result<(), service::ServiceError> {
//         if self.role != role {
//             return Err(service::ServiceError::UnAuthorized);
//         }
//         Ok(())
//     }
// }

// #[derive(Debug)]
// pub struct CoporateUser {
//     http_request: HttpRequest,
//     claims: Claims,
//     role: Role,
// }

// impl Extractor for CoporateUser {
//     fn new(http_request: HttpRequest, claims: Claims) -> Result<Self, ApiError> {
//         let role = Role::CoporateUser;
//         Ok(Self {
//             http_request,
//             claims,
//             role,
//         })
//     }

//     fn into_inner(self) -> (HttpRequest, Claims, Role) {
//         (self.http_request, self.claims, self.role)
//     }

//     fn compare_role(&self, role: Role) -> Result<(), service::ServiceError> {
//         if self.role != role {
//             return Err(service::ServiceError::UnAuthorized);
//         }
//         Ok(())
//     }
// }

// impl FromRequest for PersonalUser {
//     type Error = ApiError;
//     type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         authorized_helper::<PersonalUser, ApiError>(req)
//     }
// }

// impl FromRequest for CoporateUser {
//     type Error = ApiError;
//     type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         authorized_helper::<CoporateUser, ApiError>(req)
//     }
// }

// // SuperAdminUser => "SUPER_ADMIN_USER",
// // AdminUser => "ADMIN_USER",
// // ManagerUser => "MANAGER_USER",
// // DeveloperUser => "DEVELOPER_USER",
// // AccountUser => "ACCOUNT_USER",
// // SupportUser => "SUPPORT_USER",

// #[derive(Debug)]
// pub struct InternalSuperAdmin {
//     http_request: HttpRequest,
//     claims: Claims,
//     role: InternalRole,
// }

// #[derive(Debug)]
// pub struct InternalAdmin {
//     http_request: HttpRequest,
//     claims: Claims,
//     role: InternalRole,
// }

// #[derive(Debug)]
// pub struct InternalManager {
//     http_request: HttpRequest,
//     claims: Claims,
//     role: InternalRole,
// }

// #[derive(Debug)]
// pub struct InternalDeveloper {
//     http_request: HttpRequest,
//     claims: Claims,
//     role: InternalRole,
// }

// #[derive(Debug)]
// pub struct InternalAccount {
//     http_request: HttpRequest,
//     claims: Claims,
//     role: InternalRole,
// }

// #[derive(Debug)]
// pub struct InternalSupport {
//     http_request: HttpRequest,
//     claims: Claims,
//     role: InternalRole,
// }

// pub trait ExtractorInternal: Sized {
//     fn new(http_request: HttpRequest, claims: Claims) -> Result<Self, ApiError>;
//     fn into_inner(self) -> (HttpRequest, Claims, InternalRole);
//     fn compare_role(&self, role: InternalRole) -> Result<(), service::ServiceError>;
// }

// impl ExtractorInternal for InternalSuperAdmin {
//     fn new(http_request: HttpRequest, claims: Claims) -> Result<Self, ApiError> {
//         let role = InternalRole::SuperAdminUser;
//         Ok(Self {
//             http_request,
//             claims,
//             role,
//         })
//     }

//     fn into_inner(self) -> (HttpRequest, Claims, InternalRole) {
//         (self.http_request, self.claims, self.role)
//     }

//     fn compare_role(&self, role: InternalRole) -> Result<(), service::ServiceError> {
//         if self.role != role {
//             return Err(service::ServiceError::UnAuthorized);
//         }
//         Ok(())
//     }
// }

// impl FromRequest for InternalSuperAdmin {
//     type Error = ApiError;
//     type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         authorized_helper_internal::<InternalSuperAdmin, ApiError>(req)
//     }
// }

// impl FromRequest for InternalAdmin {
//     type Error = ApiError;
//     type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         authorized_helper_internal::<InternalAdmin, ApiError>(req)
//     }
// }

// impl ExtractorInternal for InternalAdmin {
//     fn new(http_request: HttpRequest, claims: Claims) -> Result<Self, ApiError> {
//         let role = InternalRole::AdminUser;
//         Ok(Self {
//             http_request,
//             claims,
//             role,
//         })
//     }

//     fn into_inner(self) -> (HttpRequest, Claims, InternalRole) {
//         (self.http_request, self.claims, self.role)
//     }

//     fn compare_role(&self, role: InternalRole) -> Result<(), service::ServiceError> {
//         if self.role != role {
//             return Err(service::ServiceError::UnAuthorized);
//         }
//         Ok(())
//     }
// }

// impl FromRequest for InternalManager {
//     type Error = ApiError;
//     type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         authorized_helper_internal::<InternalManager, ApiError>(req)
//     }
// }

// impl ExtractorInternal for InternalManager {
//     fn new(http_request: HttpRequest, claims: Claims) -> Result<Self, ApiError> {
//         let role = InternalRole::ManagerUser;
//         Ok(Self {
//             http_request,
//             claims,
//             role,
//         })
//     }

//     fn into_inner(self) -> (HttpRequest, Claims, InternalRole) {
//         (self.http_request, self.claims, self.role)
//     }

//     fn compare_role(&self, role: InternalRole) -> Result<(), service::ServiceError> {
//         if self.role != role {
//             return Err(service::ServiceError::UnAuthorized);
//         }
//         Ok(())
//     }
// }

// impl FromRequest for InternalDeveloper {
//     type Error = ApiError;
//     type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         authorized_helper_internal::<InternalDeveloper, ApiError>(req)
//     }
// }

// impl ExtractorInternal for InternalDeveloper {
//     fn new(http_request: HttpRequest, claims: Claims) -> Result<Self, ApiError> {
//         let role = InternalRole::DeveloperUser;
//         Ok(Self {
//             http_request,
//             claims,
//             role,
//         })
//     }

//     fn into_inner(self) -> (HttpRequest, Claims, InternalRole) {
//         (self.http_request, self.claims, self.role)
//     }

//     fn compare_role(&self, role: InternalRole) -> Result<(), service::ServiceError> {
//         if self.role != role {
//             return Err(service::ServiceError::UnAuthorized);
//         }
//         Ok(())
//     }
// }

// impl FromRequest for InternalAccount {
//     type Error = ApiError;
//     type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         authorized_helper_internal::<InternalAccount, ApiError>(req)
//     }
// }

// impl ExtractorInternal for InternalAccount {
//     fn new(http_request: HttpRequest, claims: Claims) -> Result<Self, ApiError> {
//         let role = InternalRole::AccountUser;
//         Ok(Self {
//             http_request,
//             claims,
//             role,
//         })
//     }

//     fn into_inner(self) -> (HttpRequest, Claims, InternalRole) {
//         (self.http_request, self.claims, self.role)
//     }

//     fn compare_role(&self, role: InternalRole) -> Result<(), service::ServiceError> {
//         if self.role != role {
//             return Err(service::ServiceError::UnAuthorized);
//         }
//         Ok(())
//     }
// }

// impl FromRequest for InternalSupport {
//     type Error = ApiError;
//     type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         authorized_helper_internal::<InternalSupport, ApiError>(req)
//     }
// }

// impl ExtractorInternal for InternalSupport {
//     fn new(http_request: HttpRequest, claims: Claims) -> Result<Self, ApiError> {
//         let role = InternalRole::SupportUser;
//         Ok(Self {
//             http_request,
//             claims,
//             role,
//         })
//     }

//     fn into_inner(self) -> (HttpRequest, Claims, InternalRole) {
//         (self.http_request, self.claims, self.role)
//     }

//     fn compare_role(&self, role: InternalRole) -> Result<(), service::ServiceError> {
//         if self.role != role {
//             return Err(service::ServiceError::UnAuthorized);
//         }
//         Ok(())
//     }
// }

// fn authorized_helper<
//     T: Extractor,
//     E: std::convert::From<service::ServiceError> + std::convert::From<api::ApiError>,
// >(
//     req: &HttpRequest,
// ) -> Pin<Box<dyn Future<Output = Result<T, E>>>> {
//     // lazy_static
//     let jwt_secret: String = match env::var("ENVIRONMENT") {
//         Ok(envi) if { envi.as_str() == "PRODUCTION" } => {
//             env::var("JWT_SECRET").expect("JWT_SECRET env is not set")
//         }
//         _ => JWT_DEV.to_string(),
//     };

//     let req = req.clone();
//     Box::pin(async move {
//         let jwt = jwt::extract_jwt(&req)?;
//         let claims = jwt::decode_jwt_string(jwt_secret, jwt)?;
//         let role = claims.get_role::<Role>()?;
//         let authorized = T::new(req, claims)?;
//         authorized.compare_role(role)?;

//         Ok(authorized)
//     })
// }

// fn authorized_helper_internal<
//     T: ExtractorInternal,
//     E: std::convert::From<service::ServiceError> + std::convert::From<api::ApiError>,
// >(
//     req: &HttpRequest,
// ) -> Pin<Box<dyn Future<Output = Result<T, E>>>> {
//     let jwt_secret: String = match env::var("ENVIRONMENT") {
//         Ok(envi) if { envi.as_str() == "PRODUCTION" } => {
//             env::var("JWT_SECRET").expect("JWT_SECRET env is not set")
//         }
//         _ => JWT_DEV.to_string(),
//     };

//     let req = req.clone();
//     Box::pin(async move {
//         let jwt = jwt::extract_jwt(&req)?;
//         let claims = jwt::decode_jwt_string(jwt_secret, jwt)?;
//         let role = claims.get_role::<InternalRole>()?;
//         let authorized = T::new(req, claims)?;
//         authorized.compare_role(role)?;

//         Ok(authorized)
//     })
// }

// // let jwt_secret: String = match env::var("ENVIRONMENT") {
// //     Ok(envi) if { envi.as_str() == "PRODUCTION" } => {
// //         env::var("JWT_SECRET").expect("JWT_SECRET env is not set")
// //     }
// //     _ => JWT_DEV.to_string(),
// // };

// // let req = req.clone();
// // Box::pin(async move {
// //     let jwt = jwt::extract_jwt(&req)?;
// //     let claims = jwt::decode_jwt_string(jwt_secret, jwt)?;
// //     let authorized = T::new(req, claims)?;

// //     // check if claims.role == internal_user
// //     Ok(authorized)
// // })
