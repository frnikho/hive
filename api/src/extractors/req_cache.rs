use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest, dev::Payload};
use actix_web::web::Data;
use futures::Future;
use crate::exceptions::api::ApiException;
use crate::providers::cache::CacheConnection;
use crate::state::AppState;

pub struct ReqCache {
    pub cache: CacheConnection,
}

impl FromRequest for ReqCache {
    type Error = ApiException;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let cache = request.app_data::<Data<AppState>>()
                .ok_or(ApiException::InternalError(String::from("APE-100400")))?
                .cache.clone().get_connection().unwrap();
            Ok(ReqCache {cache})
        })
    }
}