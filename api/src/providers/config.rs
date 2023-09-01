use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use crate::exceptions::config::ConfigException;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub api_workers: Option<usize>,
    pub api_port: Option<u16>,
    pub api_host: Option<String>,
    pub api_session_url: String,
    pub api_cache_url: String,
    pub api_secret: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigException<'static>> {
        dotenv().ok();
        envy::from_env::<Self>()
            .map_err(|x| x.into())
    }
}