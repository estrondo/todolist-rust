use std::fmt::Error;

use tonic::{Request, service::Interceptor};

use crate::{
    auth::{DefaultTokenReader, TokenReader},
    configuration::Configuration,
};

pub struct SecurityModule<'c> {
    _configuration: &'c Configuration,
}

impl<'c> SecurityModule<'c> {
    pub async fn new(configuration: &'c Configuration) -> Result<Self, Error> {
        Ok(Self {
            _configuration: configuration,
        })
    }

    pub fn create_auth_info_interceptor(&self) -> impl Interceptor + Clone + 'static {
        let token_reader = DefaultTokenReader;
        move |request: Request<()>| token_reader.read(request)
    }
}
