use actix_session::Session;
use bcrypt::hash;
use crate::dtos::auth::{AuthFirstLoginRequest, AuthLoginRequest};
use crate::entities::authority::{Authority, LoggedAuthority};
use crate::entities::session::{AuthoritySession, SessionValue};
use crate::entities::user::User;
use crate::exceptions::api::ApiException;
use crate::providers::cache::CacheConnection;
use crate::providers::db::DBPooled;
use crate::repositories::user_repo::{UserFindOneClause, UserRepository};
use crate::templates::template::Template;

pub struct AuthService;

impl AuthService {

    pub fn first_login(mut db: DBPooled, mut cache: CacheConnection, mut body: AuthFirstLoginRequest, session: Session) -> Result<Template<User>, ApiException> {
        body.verify_token(&mut cache)?;
        body.password = hash(body.password, 8).map_err(|_| ApiException::BadRequest(String::from("Invalid token !")))?;
        let created_user = UserRepository::create(&mut db, body.into())?;
        Self::_login("user", created_user.clone(), session)?;
        Ok(Template::new(None, Some(created_user)).with_code(201))
    }

    pub fn login(mut db: DBPooled, body: AuthLoginRequest, session: Session) -> Result<Template<User>, ApiException> {
        let user = UserRepository::find(&mut db, &UserFindOneClause::Email(body.email))?;
        user.verify_password(&body.password)
            .then(|| Self::_login("user", user.clone(), session))
            .ok_or(ApiException::BadRequest(String::from("APE-100120")))??;
        Ok(Template::new(None, Some(user)).with_code(200))
    }

    pub fn logout(auth: Authority, session: Session) -> Result<Template, ApiException> {
        session.remove(AuthoritySession::get_key().as_str())
            .ok_or(ApiException::BadRequest(String::from("APE-100120")))?;
        Ok(Template::new(Some(auth), None).with_code(200))
    }

    pub fn _login(kind: &str, user: User, session: Session) -> Result<(), ApiException> {
        session.insert(AuthoritySession::get_key().as_str(), AuthoritySession::new(kind, &user))
            .map_err(|_| ApiException::BadRequest("".to_string()))?;
        Ok(())
    }

    pub fn who_am_i(auth: Authority) -> Result<Template<LoggedAuthority>, ApiException> {
        Ok(Template::new(Some(auth.clone()), Some(LoggedAuthority::from(auth))).with_code(200))
    }

}