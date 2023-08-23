use actix_web::{HttpRequest, Responder};
use actix_web::web::ServiceConfig;
use crate::handlers::handler::Handler;

pub struct SystemHandler;

impl SystemHandler {
    pub async fn get_state() -> impl Responder {
        ""
    }

    pub async fn get_health_check(_req: HttpRequest) -> impl Responder {
        ""
    }
}

impl Handler for SystemHandler {
    fn route(cfg: &mut ServiceConfig) {
        cfg.route("/system/state/", actix_web::web::get().to(Self::get_state));
        cfg.route("/system/health/", actix_web::web::get().to(Self::get_health_check));
    }
}