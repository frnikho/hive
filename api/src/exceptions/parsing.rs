use crate::exceptions::api::ApiException;

pub enum ParsingExceptionKind {
    Datetime
}

pub struct ParsingException {
    pub kind: ParsingExceptionKind,
    pub message: String,
}

impl ParsingException {
    pub fn new(kind: ParsingExceptionKind, msg: String) -> Self {
        ParsingException {
            kind,
            message: msg,
        }
    }
}

impl Into<ApiException> for ParsingException {
    fn into(self) -> ApiException {
        match self.kind {
            ParsingExceptionKind::Datetime => ApiException::ValidationError(vec![self.message]),
        }
    }
}