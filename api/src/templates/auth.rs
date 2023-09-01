use serde_json::{json, Value};
use crate::entities::authority::{Authority, LoggedAuthority};
use crate::exceptions::api::ApiException;
use crate::templates::template::TemplateResponse;

impl TemplateResponse for LoggedAuthority {
    fn response(&self, auth: Option<Authority>) -> Result<Value, ApiException> {
        Ok(json!({
            "user": self.user.response(auth)?,
            "kind": self.kind,
            "expiration": self.expiration
        }))
    }
}