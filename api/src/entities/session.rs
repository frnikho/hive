use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::entities::user::User;

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

pub struct AccessTokenSession {
    pub access_token: String,
    pub date: NaiveDateTime,
    pub user_id: String,
}