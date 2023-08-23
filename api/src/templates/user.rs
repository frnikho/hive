use serde_json::{json, Value};
use crate::entities::authority::Authority;
use crate::entities::user::{User, UserList};
use crate::exceptions::api::ApiException;
use crate::templates::template::{TemplateListResponse, TemplateResponse};

impl TemplateResponse for User {
    fn response(&self, _authority: Option<Authority>) -> Result<Value, ApiException> {
        let a = self.created_by.as_ref().map(|x| x.response(None)).unwrap_or_else(|| Ok(Value::Null))?;
        Ok(json!({
            "id": self.id,
            "email": self.email,
            "created_date": self.created_date,
            "updated_date": self.updated_date,
            "created_by": a,
        }))
    }
}

impl TemplateListResponse for UserList {
    fn response(&self, _authority: Option<Authority>, pag: crate::entities::pagination::Pagination) -> Result<Value, ApiException> {
        let data = self.iter().map(|x| x.response(None)).collect::<Result<Vec<Value>, ApiException>>()?;
        Ok(json!({
            "data": data,
            "count": data.len(),
            "page": pag.page,
            "limit": pag.limit,
        }))
    }
}