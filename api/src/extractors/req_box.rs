use std::future::Future;
use std::pin::Pin;
use actix_session::Session;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use crate::exceptions::api::ApiException;
use crate::extractors::req_config::ReqConfig;
use crate::extractors::req_db::ReqDb;
use crate::providers::config::Config;
use crate::providers::db::DBPooled;

pub struct ReqBox {
    pub db: DBPooled,
    pub config: Config,
}

impl FromRequest for ReqBox {
    type Error = ApiException<'static>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            Ok(ReqBox {
                db: ReqDb::extract(&request).await?.pool,
                config: ReqConfig::extract(&request).await?.config,
            })
        })
    }
}