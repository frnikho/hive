use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::repositories::device_repo::{CreateDevice, UpdateDevice};

#[derive(Deserialize, Serialize, Clone, Debug, Validate)]
pub struct CreateDeviceRequest {
    #[validate(length(min = 2, max = 255))]
    name: String,
    #[validate(length(max = 4096))]
    description: Option<String>,
    active: Option<bool>,
}

impl CreateDeviceRequest {
    pub fn into_repo(self, user_id: String) -> CreateDevice {
        CreateDevice {
            name: self.name,
            description: self.description,
            is_activated: self.active.unwrap_or(false),
            created_by_user_id: Some(user_id)
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Validate)]
pub struct UpdateDeviceRequest {
    #[validate(length(min = 2, max = 255))]
    name: Option<String>,
    #[validate(length(max = 4096))]
    description: Option<String>,
    active: Option<bool>,
}

impl UpdateDeviceRequest {
    pub fn into_repo(self, user_id: String) -> UpdateDevice {
        UpdateDevice {
            name: self.name,
            description: self.description,
            is_activated: self.active,
            updated_by_user_id: Some(user_id),
        }
    }
}