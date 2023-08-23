use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::models::generated_db::sql_types::RoleType"]
pub enum RoleType {
    System,
    #[default]
    Custom,
}


#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::models::generated_db::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RoleModel {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_by_user_id: Option<String>,
    pub created_date: NaiveDateTime,
    pub updated_by_user_id: Option<String>,
    pub updated_date: Option<NaiveDateTime>,
    pub deleted_by_user_id: Option<String>,
    pub deleted_date: Option<NaiveDateTime>,
    pub is_activated: bool,
    pub is_deleted: bool,
    pub actions: Vec<Option<String>>,
    pub models: Vec<Option<String>>,
    pub triggers: Vec<Option<String>>,
    pub role_type: RoleType,
    pub key: Option<String>,
}

pub struct CreateRoleModel {
    pub name: String,
    pub description: Option<String>,
    pub created_by_user_id: Option<String>,
    pub actions: Vec<Option<String>>,
    pub models: Vec<Option<String>>,
    pub triggers: Vec<Option<String>>,
}

pub struct UpdateRoleModel {

}