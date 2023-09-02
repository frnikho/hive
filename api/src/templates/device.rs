use serde_json::{json, Value};
use crate::entities::authority::Authority;
use crate::entities::device::Device;
use crate::exceptions::api::ApiException;
use crate::templates::template::TemplateResponse;

impl TemplateResponse for Device {
    fn response(&self, auth: Option<Authority>) -> Result<Value, ApiException> {
        Ok(json!({
            "id": self.id,
            "name": self.name,
            "description": self.description,
            "created_by": self.created_by.clone().map(|x| x.response(auth.clone())).unwrap_or_else(|| Ok(Value::Null))?,
            "created_at": self.created_at,
            "updated_by": self.updated_by.clone().map(|x| x.response(auth.clone())).unwrap_or_else(|| Ok(Value::Null))?,
            "updated_at": self.updated_at,
            "deleted_by": self.deleted_by.clone().map(|x| x.response(auth.clone())).unwrap_or_else(|| Ok(Value::Null))?,
            "created_at": self.deleted_at,
            "is_activated": self.is_activated,
            "is_deleted": self.is_deleted,
        }))
    }
}