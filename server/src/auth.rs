use std::error::Error;

use prost::bytes::Bytes;
use todolist_core::model::user::UserId;
use tonic::{Request, Status};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthInfo {
    token: Bytes,
}

impl AuthInfo {
    pub fn user_id(&self) -> Result<UserId, AuthError> {
        let _token = &self.token;

        match Uuid::from_slice(_token) {
            Ok(uuid) => Ok(UserId(uuid)),
            Err(cause) => {
                log::error!("Invalid UUID token: {cause}");
                Err(AuthError::InvalidToken(
                    "Invalid token!".into(),
                    Some(Box::new(cause)),
                ))
            }
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

pub fn prepare_auth_info<A>(mut request: Request<A>) -> Result<Request<A>, Status> {
    let token = request.metadata().get_bin("authorisation-bin");
    match token {
        Some(value) => match value.to_bytes() {
            Ok(token) => {
                log::debug!("Authorisation token identified");
                request.extensions_mut().insert(AuthInfo { token });
                Ok(request)
            }
            Err(cause) => {
                log::warn!("Malformed authorisation metadata: {}", cause);
                Err(Status::unauthenticated("No authentication"))
            }
        },
        None => {
            log::debug!("No authorisation metadata");
            Ok(request)
        }
    }
}

pub(crate) fn extract_auth_info<A>(request: &Request<A>) -> Result<AuthInfo, Status> {
    let auth_info: &AuthInfo = request
        .extensions()
        .get()
        .ok_or(Status::unauthenticated("Unauthorised request"))?;

    Result::Ok(auth_info.to_owned())
}
