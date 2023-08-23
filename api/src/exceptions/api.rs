use std::fmt::Display;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;

#[derive(Debug)]
pub enum ApiException<'a> {
    BadRequest(&'a str, &'a str),
    InternalError(&'a str, &'a str),
    /// Brief.
    ///
    /// Description.
    ///
    /// * `foo` - Text about foo.
    /// * `bar` - Vector of validations errors.
    ValidationError(&'a str, Vec<String>)
}

impl Display for ApiException<'_> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ResponseError for ApiException<'_> {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiException::BadRequest(..) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            ApiException::BadRequest(..) => HttpResponse::BadRequest().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}