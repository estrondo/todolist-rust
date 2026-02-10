use std::error::Error;

pub mod api {
    pub mod v1 {
        tonic::include_proto!("todolist.v1");
    }
}

pub mod config;

pub mod services;

pub mod modules;
