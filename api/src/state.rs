use crate::providers::cache::{CacheClient, create_cache};
use crate::providers::config::Config;
use crate::providers::db::{create_pool, DBPool};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DBPool,
    pub config: Config,
    pub cache: CacheClient,
}

impl AppState {
    pub fn new() -> Result<Self, std::io::Error> {
        let config = Self::create_config()?;
        Ok (Self {
            db: Self::create_db_pool(&config)?,
            cache: Self::create_cache_client(&config)?,
            config,
        })
    }

    fn create_config() -> Result<Config, std::io::Error> {
        Config::new()
            .map_err(|x| std::io::Error::new(std::io::ErrorKind::Other, x.message()))
    }

    fn create_db_pool(config: &Config) -> Result<DBPool, std::io::Error> {
        create_pool(config.database_url.as_str())
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Error loading database pool"))
    }

    fn create_cache_client(config: &Config) -> Result<CacheClient, std::io::Error> {
        create_cache(config.api_cache_url.clone())
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Error loading cache client"))
    }
}
