use std::{env, u32};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub(crate) security: Security,
    pub server: Server,
    pub(crate) postgres: Postgres,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Postgres {
    pub username: String,
    pub password: String,
    pub address: String,
    pub database: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Dev,
    Stg,
    Prd,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Security {
    pub token_version: TokenVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TokenVersion(u8);

impl Configuration {
    /**
     * I don't know how having a default configuration, like a resource file in JVM platform.
     * Let me try it.
     */
    pub fn default() -> (Configuration, Mode) {
        match env::var("TODOLIST_MODE") {
            Ok(mode) => match mode.to_lowercase().as_ref() {
                "dev" => (Self::dev(), Mode::Dev),
                "stg" => (Self::stg(), Mode::Stg),
                "prd" => (Self::prd(), Mode::Prd),
                _ => panic!("Invalid mode {mode}."),
            },
            Err(cause) => panic!("Unable to use the TODOLIST_MODE env var due to: {cause}"),
        }
    }

    fn dev() -> Self {
        Self {
            security: Security {
                token_version: TokenVersion(0u8),
            },
            server: Server {
                port: 8080,
                address: String::from("0.0.0.0"),
            },
            postgres: Postgres {
                username: String::from("todolist"),
                password: String::from("todolist"),
                address: String::from("localhost"),
                database: String::from("todolist"),
            },
        }
    }

    fn stg() -> Self {
        Self::dev()
    }

    fn prd() -> Self {
        Self::dev()
    }
}
