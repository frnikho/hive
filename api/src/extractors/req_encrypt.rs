use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest, dev::Payload};
use actix_web::web::Data;
use futures::Future;
use crate::exceptions::api::ApiException;
use crate::providers::encrypt::EncryptProvider;
use crate::state::AppState;

pub struct ReqEncrypt(pub EncryptProvider);

impl FromRequest for ReqEncrypt {
    type Error = ApiException;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let manager = request.app_data::<Data<AppState>>()
                .ok_or(ApiException::InternalError(String::from("APE-100400")))?
                .encrypt.clone();
            Ok(ReqEncrypt(manager))
        })
    }
}