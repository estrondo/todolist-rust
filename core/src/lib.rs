use core::error::Error;

pub mod model;

pub mod persistence;

pub mod manager;

pub mod error;

pub mod convert;

pub type Result<T, E = Box<dyn Error>> = core::result::Result<T, E>;

#[cfg(test)]
mod tests;
