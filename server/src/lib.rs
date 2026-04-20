pub mod api {
    pub mod v1 {
        tonic::include_proto!("todolist.v1");
    }
}

pub mod auth;
pub mod configuration;
pub(crate) mod convert;
pub mod module;
pub mod service;
