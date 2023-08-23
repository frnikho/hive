use std::future::Future;
use std::pin::Pin;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use actix_web::web::Query;
use crate::dtos::queries::pagination::PaginationQuery;
use crate::entities::pagination::Pagination;
use crate::exceptions::api::ApiException;

pub struct ReqPagination (pub Pagination);

impl FromRequest for ReqPagination {
    type Error = ApiException;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let query: Query<PaginationQuery> = Query::extract(&request)
                .await
                .map_err(|_| ApiException::BadRequest(String::from("APE-100700")))?;
            Ok(ReqPagination (Pagination::from(query.0.into())))
        })
    }
}