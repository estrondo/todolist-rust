use std::error::Error;

use prost::bytes::{Buf, Bytes};
use todolist_core::model::user::UserId;
use tonic::{Request, Status};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

#[derive(Debug, Clone)]
pub struct AuthInfo {
    token: Bytes,
}

impl AuthInfo {
    pub fn user_id(&mut self) -> Result<UserId, AuthError> {
        let mut token = self.token.to_owned();
        match token.try_get_u128() {
            Ok(bytes) => Ok(UserId(Uuid::from_u128(bytes))),
            Err(cause) => {
                log::warn!(
                    "Unable to read the UserId from authorisation token: {}",
                    &cause
                );
                Err(AuthError::InvalidToken(
                    "UserId reading failure".into(),
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

impl<A> TryFrom<&Request<A>> for AuthInfo {
    type Error = Status;

    fn try_from(request: &Request<A>) -> Result<Self, Self::Error> {
        let info: &AuthInfo = request
            .extensions()
            .get()
            .ok_or(Status::unauthenticated("Unauthorised request"))?;
        Result::Ok(info.to_owned())
    }
}

#[cfg_attr(test, automock)]
pub(crate) trait TokenReader {
    fn read(&self, request: Request<()>) -> Result<Request<()>, Status>;
}

#[derive(Debug, Clone)]
pub(crate) struct DefaultTokenReader;

impl DefaultTokenReader {
    const AUTHORISATION_METADATA: &str = "authorisation-bin";
}

impl TokenReader for DefaultTokenReader {
    fn read(&self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let metadata = request
            .metadata()
            .get_bin(DefaultTokenReader::AUTHORISATION_METADATA);

        match metadata {
            Some(metadata) => {
                let token = metadata.to_bytes().map_err(|error| {
                    log::warn!(
                        "Malformed {} metadata: {}",
                        DefaultTokenReader::AUTHORISATION_METADATA,
                        &error
                    );
                    Status::invalid_argument(format!(
                        "Invalid {} metadata",
                        DefaultTokenReader::AUTHORISATION_METADATA
                    ))
                })?;

                request.extensions_mut().insert(AuthInfo { token });

                Ok(request)
            }
            None => Ok(request),
        }
    }
}
