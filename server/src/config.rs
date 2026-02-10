use std::{env, u32};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub server: Server,
}

#[derive(Serialize, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Dev(),
    Stg(),
    Prd(),
}

impl Configuration {
    /**
     * I don't know how having a default configuration, like a resource file in JVM platform.
     * Let me try it.
     */
    pub fn default() -> (Configuration, Mode) {
        let args: Vec<String> = env::args().collect();
        match args.get(1).map(|x| x.to_lowercase()) {
            Some(value) => match value.as_ref() {
                "dev" => (Self::dev(), Mode::Dev()),
                "stg" => (Self::stg(), Mode::Stg()),
                "prd" => (Self::prd(), Mode::Prd()),
                _ => panic!(
                    "Configuration failed: The mode '{}' is not recognised, values supported: dev, stg or prd.",
                    value
                ),
            },
            None => panic!("Configuration failed: No configuration mode."),
        }
    }

    fn dev() -> Self {
        Self {
            server: Server {
                port: 8080,
                address: String::from("0.0.0.0"),
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
