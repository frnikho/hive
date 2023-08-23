use actix_session::{SessionGetError, SessionInsertError};
use crate::exceptions::api::ApiException;

impl Into<ApiException> for SessionGetError {
    fn into(self) -> ApiException {
        ApiException::BadRequest(String::from("HIVE-1000130"))
    }
}

impl Into<ApiException> for SessionInsertError {
    fn into(self) -> ApiException {
        ApiException::Unauthorized(String::from("HIVE-1000140"))
    }
}