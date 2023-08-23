use std::future::Future;
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Json;
use serde::de::DeserializeOwned;
use validator::Validate;
use crate::exceptions::api::ApiException;

pub struct Dto<T: DeserializeOwned + Validate> {
    pub value: T
}

impl<T: DeserializeOwned + Validate + 'static> FromRequest for Dto<T> {
    type Error = ApiException<'static>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_extract = Json::<T>::from_request(req, payload);
        Box::pin(async move {
            let value = json_extract.await
                .map_err(|_| ApiException::BadRequest("Cannot extract body", "HIVE-1000500"))?.into_inner();
            value.validate()
                .map_err(|x| x.into())?;
            Ok(Dto { value})
        })
    }
}