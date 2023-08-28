use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::exceptions::db::DatabaseException;
use diesel::prelude::*;
use crate::entities::pagination::Pagination;

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

#[derive(Insertable, Debug, Default, Clone, PartialEq, Deserialize, Serialize, )]
#[diesel(table_name = crate::models::generated_db::roles)]
pub struct CreateRoleModel {
    pub name: String,
    pub description: Option<String>,
    pub created_by_user_id: Option<String>,
    pub actions: Option<Vec<String>>,
    pub models: Option<Vec<String>>,
    pub triggers: Option<Vec<String>>,
    pub role_type: RoleType,
}

#[derive(AsChangeset, Debug, Default)]
#[diesel(table_name = crate::models::generated_db::roles)]
pub struct UpdateRoleModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub actions: Option<Vec<String>>,
    pub updated_by_user_id: Option<String>,
    pub updated_date: Option<NaiveDateTime>,
    pub models: Option<Vec<String>>,
    pub triggers: Option<Vec<String>>,
}

use crate::models::generated_db::roles::dsl::*;


impl RoleModel {

    pub fn create(conn: &mut PgConnection, model: CreateRoleModel) -> Result<Self, DatabaseException> {
        diesel::insert_into(roles)
            .values(model)
            .returning(RoleModel::as_returning())
            .get_result::<Self>(conn).map_err(|x| x.into())
    }

    pub fn update(conn: &mut PgConnection, id_to_update: String, model: UpdateRoleModel) -> Result<Self, DatabaseException> {
        diesel::update(roles.filter(id.eq(id_to_update)))
            .filter(is_deleted.eq(false))
            .set(model)
            .returning(RoleModel::as_returning())
            .get_result::<Self>(conn).map_err(|x| x.into())
    }

    pub fn delete(conn: &mut PgConnection, id_to_delete: String, deleted_by: Option<String>) -> Result<Self, DatabaseException> {
        diesel::update(roles.filter(id.eq(id_to_delete)))
            .filter(is_deleted.eq(false))
            .set((is_deleted.eq(true), deleted_date.eq(Utc::now().naive_utc()), deleted_by_user_id.eq(deleted_by)))
            .returning(RoleModel::as_returning())
            .get_result::<RoleModel>(conn).map_err(|x| x.into())
    }

    pub fn find_by_id(conn: &mut PgConnection, id_to_find: String) -> Result<Self, DatabaseException> {
        roles.select(RoleModel::as_select())
            .filter(is_deleted.eq(false))
            .filter(id.eq(id_to_find))
            .first::<RoleModel>(conn).map_err(|x| x.into())
    }

    pub fn list(conn: &mut PgConnection, pag: Pagination) -> Result<Vec<Self>, DatabaseException> {
        let role = roles.select(RoleModel::as_select()).filter(is_deleted.eq(false)).into_boxed();
        match pag.bypass {
            true => role,
            false => role.offset((pag.page * pag.limit) as i64).limit(pag.limit as i64),
        }.load::<RoleModel>(conn).map_err(|x| x.into())
    }

}