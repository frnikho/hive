use crate::dtos::device::{CreateDeviceRequest, UpdateDeviceRequest};
use crate::entities::authority::Authority;
use crate::entities::device::Device;
use crate::entities::pagination::Pagination;
use crate::exceptions::api::ApiException;
use crate::extractors::req_box::ReqBox;
use crate::repositories::device_repo::{DeleteDevice, DeviceRepo, ToggleDevice};
use crate::templates::template::{Template, TemplateList};

pub struct DeviceService;

impl DeviceService {
    pub fn create(mut tool: ReqBox, auth: Authority, body: CreateDeviceRequest) -> Result<Template<Device>, ApiException> {
        let created_device = DeviceRepo::create(&mut tool.db, body.into_repo(auth.get_user().id.clone()))?;
        Ok(Template::new(Some(auth), Some(created_device)))
    }

    pub fn delete(mut tool: ReqBox, auth: Authority, device_id: &String) -> Result<Template<Device>, ApiException> {
        let deleted_device = DeviceRepo::delete(&mut tool.db, device_id, DeleteDevice {
            deleted_by_user_id: Some(auth.get_user().id.clone())
        })?;
        Ok(Template::new(Some(auth), Some(deleted_device)))
    }

    pub fn update(mut tool: ReqBox, auth: Authority, device_id: &String, body: UpdateDeviceRequest) -> Result<Template<Device>, ApiException> {
        let updated_device = DeviceRepo::update(&mut tool.db, device_id, body.into_repo(auth.get_user().id.clone()))?;
        Ok(Template::new(Some(auth), Some(updated_device)))
    }

    pub fn list(mut tool: ReqBox, auth: Authority, pag: Pagination) -> Result<TemplateList<Device>, ApiException> {
        let devices = DeviceRepo::list(&mut tool.db, &pag)?;
        Ok(TemplateList::new(Some(auth), devices, pag))
    }

    pub fn get(mut tool: ReqBox, auth: Authority, device_id: &String) -> Result<Template<Device>, ApiException> {
        let device = DeviceRepo::get(&mut tool.db, device_id)?;
        Ok(Template::new(Some(auth), Some(device)))
    }

    pub fn enable(mut tool: ReqBox, auth: Authority, device_id: &String) -> Result<Template<Device>, ApiException> {
        let updated_device = DeviceRepo::toggle(&mut tool.db, device_id, ToggleDevice {
            is_activated: true,
            updated_by_user_id: Some(auth.get_user().id.clone())
        })?;
        Ok(Template::new(Some(auth), Some(updated_device)))
    }

    pub fn disable(mut tool: ReqBox, auth: Authority, device_id: &String) -> Result<Template<Device>, ApiException> {
        let updated_device = DeviceRepo::toggle(&mut tool.db, device_id, ToggleDevice {
            is_activated: false,
            updated_by_user_id: Some(auth.get_user().id.clone())
        })?;
        Ok(Template::new(Some(auth), Some(updated_device)))
    }
}