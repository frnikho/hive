use actix_web::Responder;
use actix_web::web::{Path, ServiceConfig};
use crate::dtos::device::{CreateDeviceRequest, UpdateDeviceRequest};
use crate::extractors::req_authority::ReqAuthority;
use crate::extractors::req_box::ReqBox;
use crate::extractors::req_dto::Dto;
use crate::extractors::req_queries::ReqPagination;
use crate::handlers::handler::Handler;
use crate::services::device_service::DeviceService;

pub struct DeviceHandler;

impl DeviceHandler {

    async fn list(tool: ReqBox, auth: ReqAuthority, pag: ReqPagination) -> impl Responder {
        DeviceService::list(tool, auth.0, pag.0)
    }

    async fn create(tool: ReqBox, auth: ReqAuthority, body: Dto<CreateDeviceRequest>) -> impl Responder {
        DeviceService::create(tool, auth.0, body.value)
    }

    async fn find(tool: ReqBox, auth: ReqAuthority, device_id: Path<String>) -> impl Responder {
        DeviceService::get(tool, auth.0, &device_id.into_inner())
    }

    async fn update(tool: ReqBox, auth: ReqAuthority, body: Dto<UpdateDeviceRequest>, device_id: Path<String>) -> impl Responder {
        DeviceService::update(tool, auth.0, &device_id.into_inner(), body.value)
    }

    async fn delete(tool: ReqBox, auth: ReqAuthority, device_id: Path<String>) -> impl Responder {
        DeviceService::delete(tool, auth.0, &device_id.into_inner())
    }

    async fn enable(tool: ReqBox, auth: ReqAuthority, device_id: Path<String>) -> impl Responder {
        DeviceService::enable(tool, auth.0, &device_id.into_inner())
    }

    async fn disable(tool: ReqBox, auth: ReqAuthority, device_id: Path<String>) -> impl Responder {
        DeviceService::disable(tool, auth.0, &device_id.into_inner())
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
        cfg.route("/devices/{id}/enable/", actix_web::web::post().to(Self::enable));
        cfg.route("/devices/{id}/disable/", actix_web::web::post().to(Self::disable));
        cfg.route("/devices/{id}/history/", actix_web::web::get().to(Self::history));
        cfg.route("/devices/{id}/status/", actix_web::web::get().to(Self::status));
    }
}