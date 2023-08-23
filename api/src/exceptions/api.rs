use std::fmt::Display;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub enum ApiException {
    BadRequest(String),
    InternalError(String),
    Unauthorized(String),
    ResourceNotFound(String),
    DuplicateResource(String),
    UnknownDbError(String),
    /// Brief.
    ///
    /// Description.
    ///
    /// * `foo` - Text about foo.
    /// * `bar` - Vector of validations errors.
    ValidationError(Vec<String>)
}

impl Display for ApiException {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ResponseError for ApiException {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiException::BadRequest(..) => StatusCode::BAD_REQUEST,
            ApiException::InternalError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiException::Unauthorized(..) => StatusCode::UNAUTHORIZED,
            ApiException::ResourceNotFound(..) => StatusCode::BAD_REQUEST,
            ApiException::DuplicateResource(..) => StatusCode::BAD_REQUEST,
            ApiException::UnknownDbError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiException::ValidationError(..) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::build(self.status_code());
        match self {
            ApiException::BadRequest(x) => response.json(json!({"code": x})),
            ApiException::InternalError(x) => {
                error!("{}", x);
                HttpResponse::InternalServerError().finish()
            },
            ApiException::ValidationError(x) => response.json(x),
            ApiException::Unauthorized(x) => response.json(json!({"code": x})),
            ApiException::ResourceNotFound(x) => response.json(json!({"code": x})),
            ApiException::DuplicateResource(x) => response.json(json!({"code": x})),
            ApiException::UnknownDbError(x) => {
                error!("{}", x);
                response.finish()
            },
        }
    }
}