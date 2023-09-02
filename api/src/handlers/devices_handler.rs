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

    async fn list(_tool: ReqBox, _auth: ReqAuthority, _pag: ReqPagination) -> impl Responder {
        ""
    }
    async fn create(_tool: ReqBox, _auth: ReqAuthority, _body: Dto<CreateDeviceRequest>) -> impl Responder {
        ""
    }
    async fn find(_tool: ReqBox, _auth: ReqAuthority, _device_id: Path<String>) -> impl Responder {
        ""
    }
    async fn update(_tool: ReqBox, _auth: ReqAuthority, _body: Dto<UpdateDeviceRequest>, _device_id: Path<String>) -> impl Responder {
        ""
    }
    async fn delete(_tool: ReqBox, _auth: ReqAuthority, _device_id: Path<String>) -> impl Responder {
        ""
    }
    async fn history(_tool: ReqBox, _auth: ReqAuthority, _device_id: Path<String>) -> impl Responder {
        ""
    }
    async fn status(_tool: ReqBox, _auth: ReqAuthority, _device_id: Path<String>) -> impl Responder {
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
        cfg.route("/devices/{id}/history/", actix_web::web::get().to(Self::history));
        cfg.route("/devices/{id}/status/", actix_web::web::get().to(Self::status));
    }
}