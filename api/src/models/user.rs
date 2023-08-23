use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::exceptions::db::DatabaseException;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[diesel(belongs_to(UserModel, foreign_key = created_by_user_id))]
#[diesel(table_name = crate::models::generated_db::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: String,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub created_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub deleted_date: Option<NaiveDateTime>,
    pub created_by_user_id: Option<String>,
    pub updated_by_user_id: Option<String>,
    pub deleted_by_user_id: Option<String>,
    pub is_deleted: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::models::generated_db::users)]
pub struct CreateUserModel {
    pub email: String,
    pub firstname: String,
    pub password: String,
    pub lastname: String,
    pub created_by_user_id: Option<String>,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = crate::models::generated_db::users)]
pub struct UpdateUserModel {
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub password: Option<String>,
    pub is_deleted: Option<bool>,
    pub deleted_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by_user_id: Option<String>,
    pub deleted_by_user_id: Option<String>,
}

use crate::models::generated_db::users::dsl::*;

impl UserModel {
    pub fn find(conn: &mut PgConnection, id_to_find: String) -> Result<Self, DatabaseException> {
        users.select(UserModel::as_select())
            .filter(id.eq(id_to_find))
            .first::<UserModel>(conn).map_err(|x| x.into())
    }

}