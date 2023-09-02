use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::entities::user::User;
use crate::models::device::DeviceModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceStatus {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_by: Option<User>,
    pub updated_by: Option<User>,
    pub deleted_by: Option<User>,
    pub is_activated: bool,
    pub is_deleted: bool,
    pub status: Option<DeviceStatus>,
}

impl Device {
    pub fn add_created_user(&mut self, user: User) -> &mut Self {
        self.created_by = Some(user);
        self
    }

    pub fn add_deleted_user(&mut self, user: User) -> &mut Self {
        self.deleted_by = Some(user);
        self
    }

    pub fn add_updated_user(&mut self, user: User) -> &mut Self {
        self.updated_by = Some(user);
        self
    }
}

impl From<DeviceModel> for Device {
    fn from(model: DeviceModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            created_at: model.created_date,
            updated_at: model.updated_date,
            deleted_at: model.deleted_date,
            is_activated: model.is_activated,
            is_deleted: model.is_deleted,
            status: None,
            updated_by: None,
            deleted_by: None,
            created_by: None,
        }
    }
}