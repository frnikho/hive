use actix_web::Responder;
use actix_web::web::ServiceConfig;
use crate::handlers::handler::Handler;

pub struct UserHandler;

impl UserHandler {

    async fn list() -> impl Responder {
        ""
    }

    async fn create() -> impl Responder {
        ""
    }
    async fn find() -> impl Responder {
        ""
    }
    async fn update() -> impl Responder {
        ""
    }
    async fn delete() -> impl Responder {
        ""
    }
}

impl Handler for UserHandler {
    fn route(cfg: &mut ServiceConfig) {
        cfg.route("/users/", actix_web::web::get().to(Self::list));
        cfg.route("/users/", actix_web::web::post().to(Self::create));
        cfg.route("/users/{id}/", actix_web::web::get().to(Self::find));
        cfg.route("/users/{id}/", actix_web::web::patch().to(Self::update));
        cfg.route("/users/{id}/", actix_web::web::delete().to(Self::delete));
    }
}