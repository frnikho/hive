use actix_web::Responder;
use crate::dtos::auth::AuthLoginRequest;
use crate::providers::db::DBPooled;

pub struct AuthService;

impl AuthService {
    pub fn login(db: DBPooled, body: AuthLoginRequest) -> impl Responder {
        ""
    }
}