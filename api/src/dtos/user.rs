use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::exceptions::api::ApiException;
use crate::repositories::access_token_repo::CreateAccessToken;
use crate::repositories::user_repo::{CreateUser, UpdateUser};
use crate::utils::token::Token;

#[derive(Deserialize, Serialize, Clone, Debug, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 2, max = 255))]
    pub firstname: String,
    #[validate(length(min = 2, max = 255))]
    pub lastname: String,
}

impl CreateUserRequest {
    pub fn transform_repo(self, created_user: Option<String>) -> CreateUser {
        CreateUser {
            email: self.email,
            firstname: self.firstname,
            lastname: self.lastname,
            password: Token::RandomPassword.generate(),
            created_by_user_id: created_user
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Validate)]
pub struct UpdateUserRequest {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

impl UpdateUserRequest {
    pub fn transform_repo(self, updated_by: Option<String>) -> UpdateUser {
        UpdateUser {
            firstname: self.firstname,
            lastname: self.lastname,
            updated_date: Some(Utc::now().naive_utc()),
            updated_by_user_id: updated_by,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Validate)]
pub struct CreateAccessTokenRequest {
    #[validate(length(min = 2, max = 255))]
    pub name: String,
    pub expiration: Option<i64>
}

impl CreateAccessTokenRequest {
    pub fn transform_repo(self, key: String, expiration: Option<NaiveDateTime>) -> CreateAccessToken {
        CreateAccessToken {
            name: self.name,
            key,
            created_by_user_id: None,
            expiration,
        }
    }

    pub fn get_expiration(&self) -> Result<Option<NaiveDateTime>, ApiException> {
        Ok(match self.expiration {
            Some(x) => {
                let d = NaiveDateTime::from_timestamp_millis(x).ok_or(ApiException::ValidationError(vec![String::from("Invalid expiration format !")]))?;
                if d < Utc::now().naive_utc() {
                    return Err(ApiException::ValidationError(vec![String::from("Expiration date must be greater than now !")]));
                }
                Some(d)
            },
            None => None
        })
    }
}

/*impl CreateAccessTokenRequest {
    pub fn transform_repo(self, created_by: Option<String>) -> CreateAccessToken {
        CreateAccessToken {
            name: self.name,
            description: self.description,
            created_by_user_id: created_by,
        }
    }
}*/