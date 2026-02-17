pub mod api {
    pub mod v1 {
        tonic::include_proto!("todolist.v1");
    }
}

pub mod configuration;

pub mod service;

pub mod module;
