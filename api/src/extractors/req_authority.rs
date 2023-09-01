use std::future::Future;
use std::pin::Pin;
use actix_session::Session;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use diesel::PgConnection;
use redis::JsonCommands;
use redis_macros::Json;
use crate::entities::authority::Authority;
use crate::entities::pagination::Pagination;
use crate::entities::session::{AuthoritySession, SessionValue, TokenSession};
use crate::exceptions::api::ApiException;
use crate::extractors::req_cache::ReqCache;
use crate::extractors::req_db::ReqDb;
use crate::models::user::UserModel;
use crate::providers::cache::CacheConnection;
use crate::repositories::role_repo::RoleRepo;
use crate::repositories::user_repo::{UserFindOneClause, UserRepository};

pub struct ReqAuthority(pub Authority);

impl FromRequest for ReqAuthority {
    type Error = ApiException;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let mut db = ReqDb::extract(&request).await?.pool;
            let mut req_cache = ReqCache::extract(&request).await?;
            let sessions = Session::extract(&request).await.map_err(|_| ApiException::BadRequest(String::from("APE-100100")))?;
            let authorization_header = request.headers().get("Authorization");

            if let Some(a) = authorization_header {
                let a = a.to_str().map_err(|_| ApiException::BadRequest(String::from("APE-100100")))?;
                return Ok(ReqAuthority(extract_auth_header(&mut db, &mut req_cache.cache, a)?))
            }

            let authority_session = match sessions.get::<AuthoritySession>(AuthoritySession::get_key().as_str()).map_err(|x| {
                x.into()
            })? {
                Some(session) => session,
                None => return Err(ApiException::Unauthorized(String::from("APE-100120")))
            };

            let user = UserRepository::find(&mut db, &UserFindOneClause::Id(authority_session.clone().user_id))
                .map_err(|x| x.into())?;
            let roles = RoleRepo::for_user(&user, &mut db, &Pagination::bypass()).map_err(|x| x.into())?;
            let authority = Authority::User(user, roles, authority_session);

            Ok(ReqAuthority(authority))
        })
    }
}

pub fn extract_auth_header(db: &mut PgConnection, cache: &mut CacheConnection, value: &str) -> Result<Authority, ApiException> {
    let a = value.split("Bearer ").map(|x| x.to_string()).collect::<Vec<String>>();
    if a.len() != 2 {
        return Err(ApiException::Unauthorized(String::from("APE-100180")))
    }
    let token = a.get(1).ok_or(ApiException::Unauthorized(String::from("APE-100180")))?;
    let Json(token): Json<TokenSession> = cache.json_get(token, "$")
        .map_err(|_| ApiException::InvalidAccessToken("Invalid access token !".to_string()))?;
    if !token.is_valid() {
        return Err(ApiException::InvalidAccessToken("Invalid access token (expired)".to_string()));
    }
    let user = UserModel::find(&mut *db, &token.user_id)
        .map(|x| x.into())
        .map_err(|_| ApiException::InvalidAccessToken("Invalid access token (user not found)".to_string()))?;
    let roles = RoleRepo::for_user(&user, db, &Pagination::bypass())?;
    Ok(Authority::AccessToken(user, roles, token))
}