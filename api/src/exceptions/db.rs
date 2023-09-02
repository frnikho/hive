use diesel::result::{DatabaseErrorKind, Error};
use crate::exceptions::api::ApiException;

#[derive(Debug, Clone)]
pub enum DatabaseException {
    ConnexionException(String),
    FindException(String),
    UniqueViolation(String),
    UnknownError(String),
}

impl Into<ApiException> for DatabaseException {
    fn into(self) -> ApiException {
        match self {
            DatabaseException::FindException(msg) => ApiException::ResourceNotFound(msg.to_string()),
            DatabaseException::UnknownError(msg) => ApiException::UnknownDbError(msg.to_string()),
            DatabaseException::ConnexionException(msg) => ApiException::InternalError(msg.to_string()),
            DatabaseException::UniqueViolation(msg) => ApiException::DuplicateResource(msg.to_string()),
        }
    }
}

impl From<Error> for DatabaseException {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound => DatabaseException::FindException(String::from("Entity not found")),
            Error::DeserializationError(..) => DatabaseException::UnknownError(String::from("Deserialization Error !")),
            Error::SerializationError(..) => DatabaseException::UnknownError(String::from("Serialization Error !")),
            Error::QueryBuilderError(..) => DatabaseException::UnknownError(String::from("QueryBuilder Error !")),
            Error::RollbackTransaction => DatabaseException::UnknownError(String::from("Rollback transaction")),
            Error::AlreadyInTransaction => DatabaseException::UnknownError(String::from("Already in transaction")),
            Error::DatabaseError(err, _) => match err {
                DatabaseErrorKind::UniqueViolation => DatabaseException::UniqueViolation(String::from("Entity already exists")),
                DatabaseErrorKind::UnableToSendCommand => DatabaseException::UnknownError(String::from("Unable to send command to database")),
                DatabaseErrorKind::CheckViolation => DatabaseException::UnknownError(String::from("Check violation")),
                DatabaseErrorKind::ForeignKeyViolation => DatabaseException::UnknownError(String::from("Foreign key violation")),
                DatabaseErrorKind::SerializationFailure => DatabaseException::UnknownError(String::from("Serialization failure")),
                DatabaseErrorKind::ReadOnlyTransaction => DatabaseException::ConnexionException(String::from("Unable to acquire connection")),
                _ => DatabaseException::UnknownError(String::from("Unknown database error")),
            }
            _ => DatabaseException::UnknownError(String::from("Unknown database error")),
        }
    }
}

impl<'a> Into<ApiException> for r2d2::Error {
    fn into(self) -> ApiException {
        ApiException::InternalError(String::from("HIVE_1000100"))
    }
}
