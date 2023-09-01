use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::entities::role::RoleList;
use crate::entities::session::{AuthoritySession, TokenSession};
use crate::entities::user::User;

#[derive(Debug, Clone)]
pub enum Authority {
    User(User, RoleList, AuthoritySession),
    AccessToken(User, RoleList, TokenSession),
}

impl Authority {
    pub fn get_user(&self) -> &User {
        match self {
            Authority::User(user, ..) => user,
            Authority::AccessToken(user, ..) => user,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggedAuthority {
    pub user: User,
    pub kind: String,
    pub expiration: Option<NaiveDateTime>,
}

impl From<Authority> for LoggedAuthority {
    fn from(auth: Authority) -> Self {
        let kind = match &auth {
            Authority::User(..) => "user".to_string(),
            Authority::AccessToken(..) => "access_token".to_string()
        };
        let expiration = match &auth {
            Authority::User(_, _, a) => Some(a.date),
            Authority::AccessToken(_, _, a) => a.get_expiration()
        };
        Self {
            kind,
            expiration,
            user: auth.get_user().clone(),
        }
    }
}