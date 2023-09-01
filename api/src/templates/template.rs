use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::http::StatusCode;
use serde::Serialize;
use serde_json::Value;
use crate::entities::authority::Authority;
use crate::entities::pagination::Pagination;
use crate::exceptions::api::ApiException;

pub trait TemplateResponse {
    fn response(&self, authority: Option<Authority>) -> Result<Value, ApiException>;
}

pub trait TemplateListResponse {
    fn response(&self, authority: Option<Authority>, pagination: Pagination) -> Result<Value, ApiException>;
}

pub struct EmptyTemplate;

impl TemplateResponse for EmptyTemplate {
    fn response(&self, _authority: Option<Authority>) -> Result<Value, ApiException> {
        Ok(Value::Null)
    }
}

pub struct Template<T = EmptyTemplate> {
    pub code: Option<u16>,
    pub data: Option<T>,
    pub authority: Option<Authority>
}

impl<T> Template<T> where T: TemplateResponse {
    pub fn new(authority: Option<Authority>, value: Option<T>) -> Self {
        Template {
            code: None,
            data: value,
            authority
        }
    }

    pub fn with_code(mut self, code: u16) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_authority(mut self, authority: Authority) -> Self {
        self.authority = Some(authority);
        self
    }
}

impl<T> Responder for Template<T> where T: TemplateResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut response = HttpResponse::build(StatusCode::from_u16(self.code.unwrap_or(200)).unwrap_or_default());
        let data = if let Some(d) = self.data {
            d
        } else {
            return response.finish();
        };
        data.response(self.authority).map(|data| {
            response.json(data)
        }).unwrap_or_else(|x| {
            x.error_response()
        })
    }
}

pub struct TemplateList<T: TemplateResponse> {
    pub code: Option<u16>,
    pub data: Vec<T>,
    pub authority: Option<Authority>,
    pub pagination: Pagination
}

impl<T> TemplateList<T> where T: TemplateResponse {
    pub fn new(authority: Option<Authority>, value: Vec<T>, pagination: Pagination) -> Self {
        TemplateList {
            code: None,
            data: value,
            authority,
            pagination
        }
    }

    pub fn with_code(mut self, code: u16) -> Self {
        self.code = Some(code);
        self
    }

    pub fn with_authority(mut self, authority: Authority) -> Self {
        self.authority = Some(authority);
        self
    }
}

#[derive(Serialize)]
struct TR<T: Serialize> {
    data: T,
    limit: u16,
    page: u16,
    count: usize,
}

impl<T> Responder for TemplateList<T> where T: TemplateResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut response = HttpResponse::build(StatusCode::from_u16(self.code.unwrap_or(200)).unwrap_or_default());
        let data = self.data.iter().map(|x| x.response(None)).collect::<Result<Vec<Value>, ApiException>>();
        match data {
            Ok(data) => {
                response.json(TR {
                    count: data.len(),
                    data,
                    limit: self.pagination.limit,
                    page: self.pagination.page,
                })
            },
            Err(x) => x.error_response()
        }
    }
}