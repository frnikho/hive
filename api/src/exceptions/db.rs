use diesel::result::{DatabaseErrorKind, Error};
use crate::exceptions::api::ApiException;

#[derive(Debug, Clone)]
pub enum DatabaseException<'a> {
    ConnexionException(&'a str),
    FindException(&'a str),
    UnknownError(&'a str),
}

impl<'a> Into<DatabaseException<'a>> for Error {
    fn into(self) -> DatabaseException<'a> {
        match self {
            Error::NotFound => DatabaseException::FindException("Cannot find user !"),
            Error::DeserializationError(..) => DatabaseException::UnknownError("Deserialization Error !"),
            Error::DatabaseError(err, _) => match err {
                DatabaseErrorKind::UniqueViolation => DatabaseException::UnknownError(""),
                _ => DatabaseException::UnknownError("Unknown database error"),
            }
            _ => DatabaseException::UnknownError("Unknown database error"),
        }
    }
}

impl<'a> Into<ApiException<'a>> for DatabaseException<'a> {
    fn into(self) -> ApiException<'a> {
        match self {
            DatabaseException::FindException(msg) => ApiException::BadRequest(msg, ""),
            DatabaseException::UnknownError(msg) => ApiException::BadRequest(msg, ""),
            _ => todo!()
        }
    }
}

impl<'a> Into<ApiException<'a>> for r2d2::Error {
    fn into(self) -> ApiException<'a> {
        ApiException::InternalError("Error loading database pool", "HIVE_1000100")
    }
}
