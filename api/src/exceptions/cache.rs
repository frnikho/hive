use std::fmt::Display;
use redis::RedisError;
use crate::exceptions::api::ApiException;

#[derive(Debug)]
pub struct CacheException {
    pub message: String
}

impl Display for CacheException {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Into<CacheException> for RedisError {
    fn into(self) -> CacheException {
        CacheException {
            message: String::from("CacheException")
        }
    }
}

impl Into<ApiException> for CacheException {
    fn into(self) -> ApiException {
        ApiException::BadRequest(self.message)
    }
}