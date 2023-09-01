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
    InvalidAccessToken(String),
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

/*impl From<diesel::result::Error> for ApiException {
    fn from(err: diesel::result::Error) -> Self {
        return match err {
            diesel::result::Error::DatabaseError(_, err) => {
                match err.message().to_lowercase().as_str() {
                    "unique_violation" => {
                        let msg = err.message().to_lowercase();
                        if msg.contains("email") {
                            return ApiException::DuplicateResource(String::from("APE-100200"))
                        }
                        if msg.contains("username") {
                            return ApiException::DuplicateResource(String::from("APE-100210"))
                        }
                        ApiException::UnknownDbError(String::from("APE-100220"))
                    },
                    _ => ApiException::UnknownDbError(String::from("APE-100230"))
                }
            },
            _ => ApiException::UnknownDbError(String::from("APE-100240"))
        }
    }
}*/

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
            ApiException::InvalidAccessToken(..) => StatusCode::UNAUTHORIZED,
            /*_ => StatusCode::IM_A_TEAPOT*/
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
            ApiException::InvalidAccessToken(x) => response.json(json!({"message": x, "code": "APE-100180"})),
            /*_ => response.json(json!({"code": "IM_A_TEAPOT"})),*/
        }
    }
}