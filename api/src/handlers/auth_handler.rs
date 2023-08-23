use actix_session::Session;
use actix_web::Responder;
use actix_web::web::ServiceConfig;
use crate::dtos::auth::AuthLoginRequest;
use crate::extractors::req_box::ReqBox;
use crate::extractors::req_dto::Dto;
use crate::handlers::handler::Handler;
use crate::services::auth_service::AuthService;

pub struct AuthHandler;

impl AuthHandler {

    async fn login(tool: ReqBox, body: Dto<AuthLoginRequest>, session: Session) -> impl Responder {
        AuthService::login(tool.db, body.value)
    }

    async fn logout(tool: ReqBox, session: Session) -> impl Responder {
        ""
    }

}

impl Handler for AuthHandler {
    fn route(cfg: &mut ServiceConfig) {
        cfg.route("/auth/login/", actix_web::web::post().to(Self::login));
        cfg.route("/auth/logout/", actix_web::web::post().to(Self::logout));
    }
}