use std::future::Future;
use std::pin::Pin;
use actix_session::Session;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use crate::entities::authority::{Authority, AuthoritySession};
use crate::entities::session::SessionValue;
use crate::exceptions::api::ApiException;
use crate::extractors::req_db::ReqDb;
use crate::repositories::user_repo::{UserFindOneClause, UserRepository};

pub struct ReqAuthority(pub Authority);

impl FromRequest for ReqAuthority {
    type Error = ApiException;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let mut db = ReqDb::extract(&request).await?.pool;
            let sessions = Session::extract(&request).await.map_err(|_| ApiException::BadRequest(String::from("APE-100100")))?;

            let authority_session = match sessions.get::<AuthoritySession>(AuthoritySession::get_key().as_str()).map_err(|x| {
                println!("{:?}", x);
                x.into()
            })? {
                Some(session) => session,
                None => return Err(ApiException::Unauthorized(String::from("APE-100120")))
            };

            let user = UserRepository::find(&mut db, UserFindOneClause::Id(authority_session.user_id))
                .map_err(|x| x.into())?;
            let authority = Authority::User(user);

            Ok(ReqAuthority(authority))
        })
    }
}