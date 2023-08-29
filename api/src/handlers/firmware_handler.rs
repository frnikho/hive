use actix_web::Responder;
use actix_web::web::ServiceConfig;
use crate::handlers::handler::Handler;

pub struct FirmwareHandler;

impl FirmwareHandler {

    pub async fn create() -> impl Responder {
        ""
    }

    pub async fn get() -> impl Responder {
        ""
    }

    pub async fn list() -> impl Responder {
        ""
    }

    pub async fn update() -> impl Responder {
        ""
    }

    pub async fn delete() -> impl Responder {
        ""
    }

    pub async fn get_bin() -> impl Responder {
        ""
    }

    pub async fn upload_bin() -> impl Responder {
        ""
    }

}

impl Handler for FirmwareHandler {
    fn route(cfg: &mut ServiceConfig) {
        cfg.route("/firmwares/", actix_web::web::post().to(Self::create));
        cfg.route("/firmwares/", actix_web::web::get().to(Self::list));
        cfg.route("/firmwares/{id}/", actix_web::web::get().to(Self::get));
        cfg.route("/firmwares/{id}/", actix_web::web::delete().to(Self::delete));
        cfg.route("/firmwares/{id}/", actix_web::web::patch().to(Self::update));
        cfg.route("/firmwares/{id}/bin/", actix_web::web::get().to(Self::get_bin));
        cfg.route("/firmwares/{id}/bin/", actix_web::web::post().to(Self::upload_bin));
    }
}