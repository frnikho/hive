use serde_json::{json, Value};
use crate::entities::access_token::AccessToken;
use crate::entities::authority::Authority;
use crate::exceptions::api::ApiException;
use crate::templates::template::TemplateResponse;

impl TemplateResponse for AccessToken {
    fn response(&self, _authority: Option<Authority>) -> Result<Value, ApiException> {
        Ok(json!(self))
    }
}