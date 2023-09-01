use chrono::{NaiveDateTime, Utc};
use redis::JsonCommands;
use redis_macros::Json;
use serde::{Deserialize, Serialize};
use crate::entities::access_token::AccessToken;
use crate::entities::user::User;
use crate::exceptions::cache::CacheException;
use crate::providers::cache::CacheConnection;

pub trait SessionValue {
    fn get_key() -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthoritySession {
    pub id: String,
    pub user_id: String,
    pub kind: String,
    pub date: NaiveDateTime,
}

impl AuthoritySession {
    pub fn new(kind: &str, user: &User) -> Self {
        AuthoritySession {
            id: user.id.clone(),
            user_id: user.id.clone(),
            kind: kind.to_string(),
            date: Utc::now().naive_utc(),
        }
    }
}


impl SessionValue for AuthoritySession {
    fn get_key() -> String {
        "authority".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenSessionKind {
    AccessToken(AccessToken),
    OauthToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSession {
    pub kind: TokenSessionKind,
    pub user_id: String,
}

impl TokenSession {
    pub fn new(kind: TokenSessionKind, user: &User) -> Self {
        TokenSession {
            kind,
            user_id: user.id.clone(),
        }
    }

    pub fn get_key(&self) -> String {
        match self {
            TokenSession {
                kind: TokenSessionKind::AccessToken(token),
                user_id: _,
            } => token.key.clone(),
            TokenSession {
                kind: TokenSessionKind::OauthToken,
                user_id: _,
            } => todo!("Oauth"),
        }
    }
    
    pub fn get_expiration(&self) -> Option<NaiveDateTime> {
        match self {
            TokenSession {
                kind: TokenSessionKind::AccessToken(token),
                user_id: _,
            } => token.expiration,
            TokenSession {
                kind: TokenSessionKind::OauthToken,
                user_id: _,
            } => None,
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            TokenSession {
                kind: TokenSessionKind::AccessToken(token),
                user_id: _,
            } => !token.is_expired(),
            TokenSession {
                kind: TokenSessionKind::OauthToken,
                user_id: _,
            } => true,
        }
    }

    pub fn save_to_cache(&self, cache: &mut CacheConnection) -> Result<(), CacheException> {
        cache.json_set(self.get_key().clone(), "$", self)
            .map_err(|x| x.into())?;
        Ok(())
    }

    pub fn get_from_cache(cache: &mut CacheConnection, key: &str) -> Result<Self, CacheException> {
        let Json(token): Json<Self> = cache.json_get(key, "$")
            .map_err(|x| x.into())?;
        Ok(token)
    }

    pub fn delete_from_cache(cache: &mut CacheConnection, key: &str) -> Result<(), CacheException> {
        cache.json_del(key, "$")
            .map_err(|x| x.into())?;
        Ok(())
    }
}