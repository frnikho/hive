use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest, dev::Payload};
use actix_web::web::Data;
use futures::Future;
use crate::exceptions::api::ApiException;
use crate::providers::db::DBPooled;
use crate::state::AppState;

pub struct ReqDb {
    pub pool: DBPooled,
}

impl FromRequest for ReqDb {
    type Error = ApiException<'static>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let pool = request.app_data::<Data<AppState>>()
                .ok_or(ApiException::InternalError("Cannot get application state !", "APE-100300"))?
                .db.get()
                .map_err(|x| x.into())?;
            Ok(ReqDb {pool})
        })
    }
}