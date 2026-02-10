use core::error::Error;

pub mod persistence;

pub mod manager;

pub mod model;

pub type Result<T> = core::result::Result<T, Box<dyn Error>>;
