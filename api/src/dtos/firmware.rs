use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct CreateFirmwareRequest {
    pub name: String,
    pub description: Option<String>,
    pub version: String,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct UpdateFirmwareRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct UploadFirmwareRequest {

}