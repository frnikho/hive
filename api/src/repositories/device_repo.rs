use chrono::Utc;
use diesel::PgConnection;
use crate::entities::device::{Device, DeviceStatus};
use crate::entities::pagination::Pagination;
use crate::exceptions::api::ApiException;
use crate::exceptions::db::DatabaseException;
use crate::models::device::{CreateDeviceModel, DeviceModel, UpdateDeviceModel};
use crate::repositories::user_repo::{UserFindOneClause, UserRepository};

pub struct DeviceRepo;

pub struct CreateDevice {
    pub name: String,
    pub description: Option<String>,
    pub is_activated: bool,
    pub created_by_user_id: Option<String>,
}

impl CreateDevice {
    pub fn into_model(self) -> CreateDeviceModel {
        CreateDeviceModel {
            name: self.name,
            description: self.description,
            is_activated: self.is_activated,
            created_by_user_id: self.created_by_user_id
        }
    }
}

pub struct UpdateDevice {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_activated: Option<bool>,
    pub updated_by_user_id: Option<String>,
}

impl UpdateDevice {
    pub fn into_model(self) -> UpdateDeviceModel {
        UpdateDeviceModel {
            name: self.name,
            description: self.description,
            is_activated: self.is_activated,
            updated_by_user_id: self.updated_by_user_id,
            updated_date: Some(Utc::now().naive_utc()),
            ..Default::default()
        }
    }
}

pub struct DeleteDevice {
    pub deleted_by_user_id: Option<String>,
}

impl DeleteDevice {
    pub fn into_model(self) -> UpdateDeviceModel {
        UpdateDeviceModel {
            is_deleted: Some(true),
            deleted_by_user_id: self.deleted_by_user_id,
            deleted_date: Some(Utc::now().naive_utc()),
            ..Default::default()
        }
    }
}

pub struct CreateDeviceStatus {
    pub device_id: String,
    pub status: String,
}

pub struct ToggleDevice {
    pub is_activated: bool,
    pub updated_by_user_id: Option<String>,
}

impl ToggleDevice {
    pub fn into_model(self) -> UpdateDeviceModel {
        UpdateDeviceModel {
            is_activated: Some(self.is_activated),
            updated_by_user_id: self.updated_by_user_id,
            ..Default::default()
        }
    }
}

impl DeviceRepo {

    pub fn create(conn: &mut PgConnection, body: CreateDevice) -> Result<Device, ApiException> {
        DeviceModel::create(conn, &body.into_model())
            .map(|model| model.into())
            .map_err(|x| x.into())
    }

    pub fn delete(conn: &mut PgConnection, device_id: &String, body: DeleteDevice) -> Result<Device, ApiException> {
        DeviceModel::update(conn, device_id, &body.into_model())
            .map(|model| model.into())
            .map_err(|x| x.into())

    }

    pub fn update(conn: &mut PgConnection, device_id: &String, body: UpdateDevice) -> Result<Device, ApiException> {
        DeviceModel::update(conn, device_id, &body.into_model())
            .map(|model| model.into())
            .map_err(|x| x.into())

    }

    pub fn list(conn: &mut PgConnection, pag: &Pagination) -> Result<Vec<Device>, ApiException> {
        DeviceModel::list(conn, pag)
            .map(|models| models.into_iter().map(|model| model.into()).collect())
            .map_err(|x| x.into())

    }

    pub fn get(conn: &mut PgConnection, device_id: &String) -> Result<Device, ApiException> {
        let model = DeviceModel::find_by_id(conn, device_id)
            .map_err(|x| x.into())?;
        let mut device: Device = model.clone().into();
        if let Some(user) = model.created_by_user_id {
            device.add_created_user(UserRepository::find(conn, &UserFindOneClause::Id(user))?);
        }
        if let Some(user) = model.updated_by_user_id {
            device.add_updated_user(UserRepository::find(conn, &UserFindOneClause::Id(user))?);
        }
        if let Some(user) = model.deleted_by_user_id {
            device.add_updated_user(UserRepository::find(conn, &UserFindOneClause::Id(user))?);
        }
        Ok(device)
    }

    pub fn toggle(conn: &mut PgConnection, device_id: &String, body: ToggleDevice) -> Result<Device, ApiException> {
        DeviceModel::update(conn, device_id, &body.into_model())
            .map(|model| model.into())
            .map_err(|x| x.into())

    }

    pub fn get_status(_conn: &mut PgConnection, _device_id: &String) -> Result<DeviceStatus, ApiException> {
        todo!("Implement DeviceRepo::get_status")
    }

    pub fn add_status() {

    }

    pub fn list_status(_conn: &mut PgConnection, _device_id: &String, _pag: &Pagination) -> Result<DeviceStatus, DatabaseException> {
        todo!("Implement DeviceRepo::list_status")
    }

}