use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest, dev::Payload};
use actix_web::web::Data;
use futures::Future;
use crate::exceptions::api::ApiException;
use crate::providers::config::Config;
use crate::state::AppState;

pub struct ReqConfig {
    pub config: Config,
}

impl FromRequest for ReqConfig {
    type Error = ApiException<'static>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let config = request.app_data::<Data<AppState>>()
                .ok_or(ApiException::InternalError("Cannot get application state !", "APE-100300"))?
                .config.clone();
            Ok(ReqConfig {config})
        })
    }
}