use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::entities::pagination::Pagination;
use crate::exceptions::db::DatabaseException;
use crate::models::generated_db::devices::dsl::devices;
use crate::models::generated_db::devices::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[diesel(belongs_to(DeviceModel, foreign_key = created_by_user_id))]
#[diesel(table_name = crate::models::generated_db::devices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DeviceModel {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_by_user_id: Option<String>,
    pub updated_by_user_id: Option<String>,
    pub deleted_by_user_id: Option<String>,
    pub created_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub deleted_date: Option<NaiveDateTime>,
    pub is_deleted: bool,
    pub is_activated: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::models::generated_db::devices)]
pub struct CreateDeviceModel {
    pub name: String,
    pub is_activated: bool,
    pub description: Option<String>,
    pub created_by_user_id: Option<String>,
}

#[derive(AsChangeset, Debug, Default)]
#[diesel(table_name = crate::models::generated_db::devices)]
pub struct UpdateDeviceModel {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_activated: Option<bool>,
    pub is_deleted: Option<bool>,
    pub deleted_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by_user_id: Option<String>,
    pub deleted_by_user_id: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[diesel(belongs_to(DeviceModel, foreign_key = device_id))]
#[diesel(table_name = crate::models::generated_db::devices_status)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DeviceStatusModel {
    pub id: String,
    pub device_id: String,
    pub status: String,
    pub created_date: NaiveDateTime,
}

impl DeviceModel {

    pub fn find_by_id(conn: &mut PgConnection, device_id: &String) -> Result<Self, DatabaseException> {
        devices
            .filter(id.eq(device_id))
            .filter(is_deleted.eq(false))
            .select(DeviceModel::as_select())
            .first::<Self>(conn)
            .map_err(|err| err.into())
    }

    pub fn list(conn: &mut PgConnection, pag: &Pagination) -> Result<Vec<Self>, DatabaseException> {
        let all_devices = devices.select(DeviceModel::as_select()).filter(is_deleted.eq(false)).into_boxed();
        match pag.bypass {
            true => all_devices,
            false => all_devices.offset((pag.page * pag.limit) as i64).limit(pag.limit as i64),
        }.load::<DeviceModel>(conn).map_err(|x| x.into())
    }

    pub fn create(conn: &mut PgConnection, new_device: &CreateDeviceModel) -> Result<Self, DatabaseException> {
        diesel::insert_into(devices)
            .values(new_device)
            .returning(DeviceModel::as_returning())
            .get_result::<DeviceModel>(conn).map_err(|x| x.into())
    }

    pub fn update(conn: &mut PgConnection, device_id: &String, update_device: &UpdateDeviceModel) -> Result<Self, DatabaseException> {
        diesel::update(devices.filter(id.eq(device_id)))
            .filter(is_deleted.eq(false))
            .set(update_device)
            .returning(DeviceModel::as_returning())
            .get_result::<DeviceModel>(conn).map_err(|x| {
            println!("{:?}", x);
            x.into()
        })
    }

}