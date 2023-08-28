use actix_web::Responder;
use actix_web::web::{Path, ServiceConfig};
use crate::dtos::device::{CreateDeviceRequest, UpdateDeviceRequest};
use crate::extractors::req_authority::ReqAuthority;
use crate::extractors::req_box::ReqBox;
use crate::extractors::req_dto::Dto;
use crate::extractors::req_queries::ReqPagination;
use crate::handlers::handler::Handler;

pub struct DeviceHandler;

impl DeviceHandler {

    async fn list(tool: ReqBox, auth: ReqAuthority, pag: ReqPagination) -> impl Responder {
        ""
    }
    async fn create(tool: ReqBox, auth: ReqAuthority, body: Dto<CreateDeviceRequest>) -> impl Responder {
        ""
    }
    async fn find(tool: ReqBox, auth: ReqAuthority, device_id: Path<String>) -> impl Responder {
        ""
    }
    async fn update(tool: ReqBox, auth: ReqAuthority, body: Dto<UpdateDeviceRequest>, device_id: Path<String>) -> impl Responder {
        ""
    }
    async fn delete(tool: ReqBox, auth: ReqAuthority, device_id: Path<String>) -> impl Responder {
        ""
    }
}

impl Handler for DeviceHandler {
    fn route(cfg: &mut ServiceConfig) {
        cfg.route("/devices/", actix_web::web::get().to(Self::list));
        cfg.route("/devices/", actix_web::web::post().to(Self::create));
        cfg.route("/devices/{id}/", actix_web::web::get().to(Self::find));
        cfg.route("/devices/{id}/", actix_web::web::patch().to(Self::update));
        cfg.route("/devices/{id}/", actix_web::web::delete().to(Self::delete));
    }
}