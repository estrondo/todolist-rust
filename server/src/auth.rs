use std::{error::Error, io::Read};

use prost::bytes::Buf;
use sea_orm::prelude::Uuid;
use todolist_core::model::user::UserId;
use tonic::{Request, Status};

#[derive(Debug, Clone)]
pub struct AuthToken(pub [u8; 2], pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct AuthInfo {
    _token: AuthToken,
}

impl AuthInfo {
    pub fn user_id(&self) -> Result<UserId, AuthError> {
        let data = &self._token.0;
        let mut buf = [0u8; 16];
        let x = data.reader().read(&mut buf).map_err(|cause| {
            AuthError::InvalidToken(
                "Unable to extract UserId from Token.".to_owned(),
                Some(Box::new(cause)),
            )
        })?;

        if x == 16 {
            Ok(UserId(Uuid::from_bytes(buf)))
        } else {
            Err(AuthError::InvalidToken(
                "Token is too short.".to_owned(),
                None,
            ))
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken(String, Option<Box<dyn Error>>),
}

impl From<AuthError> for Status {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::InvalidToken(message, _) => Status::unauthenticated(message),
        }
    }
}

pub fn extract_auth_info<A>(request: &Request<A>) -> Result<AuthInfo, Status> {
    let auth_info: &AuthInfo = request
        .extensions()
        .get()
        .ok_or(Status::unauthenticated("Unauthenticated request!"))?;

    Result::Ok(auth_info.clone())
}
