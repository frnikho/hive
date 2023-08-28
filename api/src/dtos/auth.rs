use redis::JsonCommands;
use redis_macros::Json;
use serde::Deserialize;
use validator::Validate;
use crate::exceptions::api::ApiException;
use crate::providers::cache::CacheConnection;
use crate::repositories::user_repo::CreateUser;

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct AuthLoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 255))]
    pub password: String,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct AuthFirstLoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 255))]
    pub password: String,
    #[validate(length(min = 2, max = 255))]
    pub firstname: String,
    #[validate(length(min = 2, max = 255))]
    pub lastname: String,
    #[validate(length(min = 8, max = 255))]
    pub token: String,
}

impl AuthFirstLoginRequest {
    pub fn verify_token(&self, cache: &mut CacheConnection) -> Result<(), ApiException> {
        let Json(ab): Json<String> = cache.json_get("super_user_token", "$")
            .map_err(|_| ApiException::BadRequest(String::from("Cannot get super user token !")))?;
        if ab != self.token {
            return Err(ApiException::BadRequest(String::from("Invalid token")));
        }
        Ok(())
    }
}

impl Into<CreateUser> for AuthFirstLoginRequest {
    fn into(self) -> CreateUser {
        CreateUser {
            email: self.email,
            password: self.password,
            firstname: self.firstname,
            lastname: self.lastname,
            created_by_user_id: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct OauthCodeRequest {

}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct OauthTokenRequest {

}