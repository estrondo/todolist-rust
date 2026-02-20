use std::{
    error::Error,
    fmt::{Display, Write},
};

use todolist_core::Result;
use tonic::{Request, Status};

#[derive(Debug, Default, Clone)]
struct Token(Vec<u8>);

#[derive(Debug, Default, Clone)]
pub struct AuthInfo {
    token: Token,
}

#[derive(Debug)]
pub struct AuthFailure {
    message: String,
}

impl Error for AuthFailure {}

impl Display for AuthFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("AuthFailure: {}", self.message).as_ref())
    }
}

pub fn extract_auth_info<A>(request: &Request<A>) -> Result<AuthInfo, Status> {
    let auth_info: &AuthInfo = request
        .extensions()
        .get()
        .ok_or(Status::unauthenticated("Unauthenticated request!"))?;

    Result::Ok(auth_info.clone())
}
