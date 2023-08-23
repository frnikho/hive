use redis::JsonCommands;
use tracing::info;
use crate::utils::token::Token;

pub type CacheClient = redis::Client;
pub type CacheConnection = redis::Connection;

pub fn create_cache(redis_cache_url: String) -> Result<CacheClient, std::io::Error> {
    redis::Client::open(redis_cache_url)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Error loading database pool"))
}

pub fn create_super_user_token(con: &mut CacheConnection) -> Result<String, std::io::Error> {
    let token = Token::SuperUserToken.generate();
    con.json_set("super_user_token", "$", &token)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Cannot create super user token !"))?;
    info!("=============================");
    info!("Super user token created: ");
    info!("{}", token);
    info!("=============================");
    Ok(token)
}