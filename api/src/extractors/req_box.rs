use std::future::Future;
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use crate::exceptions::api::ApiException;
use crate::extractors::req_cache::ReqCache;
use crate::extractors::req_config::ReqConfig;
use crate::extractors::req_db::ReqDb;
use crate::extractors::req_encrypt::ReqEncrypt;
use crate::providers::cache::CacheConnection;
use crate::providers::config::Config;
use crate::providers::db::DBPooled;
use crate::providers::encrypt::EncryptProvider;

pub struct ReqBox {
    pub db: DBPooled,
    pub config: Config,
    pub cache: CacheConnection,
    pub encrypt: EncryptProvider,
}

impl FromRequest for ReqBox {
    type Error = ApiException;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            Ok(ReqBox {
                db: ReqDb::extract(&request).await?.pool,
                config: ReqConfig::extract(&request).await?.config,
                cache: ReqCache::extract(&request).await?.cache,
                encrypt: ReqEncrypt::extract(&request).await?.0,
            })
        })
    }
}