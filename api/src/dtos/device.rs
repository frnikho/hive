use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Clone, Debug, Validate)]
pub struct CreateDeviceRequest {
    #[validate(length(min = 2, max = 255))]
    name: String,
    #[validate(length(max = 4096))]
    description: Option<String>,
    active: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Validate)]
pub struct UpdateDeviceRequest {
    #[validate(length(min = 2, max = 255))]
    name: Option<String>,
    #[validate(length(max = 4096))]
    description: Option<String>,
    active: Option<bool>,
}