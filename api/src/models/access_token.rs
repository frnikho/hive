use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::exceptions::db::DatabaseException;
use crate::models::generated_db::access_token::dsl::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[diesel(belongs_to(AccessTokenModel, foreign_key = created_by_user_id))]
#[diesel(table_name = crate::models::generated_db::access_token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccessTokenModel {
    pub id: String,
    pub name: String,
    pub key: String,
    pub created_date: NaiveDateTime,
    pub deleted_date: Option<NaiveDateTime>,
    pub created_by_user_id: String,
    pub is_deleted: bool,
    pub expiration: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::models::generated_db::access_token)]
pub struct CreateAccessTokenModel {
    pub name: String,
    pub expiration: Option<NaiveDateTime>,
    pub key: String,
    pub created_by_user_id: String,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = crate::models::generated_db::access_token)]
pub struct DeleteAccessTokenModel {
    pub deleted_date: NaiveDateTime,
    pub is_deleted: bool,
}

impl DeleteAccessTokenModel {
    pub fn new() -> Self {
        Self {
            deleted_date: Utc::now().naive_utc(),
            is_deleted: true
        }
    }
}

impl AccessTokenModel {

    pub fn create(conn: &mut PgConnection, body: CreateAccessTokenModel) -> Result<Self, DatabaseException> {
        diesel::insert_into(access_token)
            .values(body)
            .returning(AccessTokenModel::as_returning())
            .get_result::<Self>(conn).map_err(|x| x.into())
    }

    pub fn delete(conn: &mut PgConnection, id_to_delete: String, body: DeleteAccessTokenModel) -> Result<Self, DatabaseException> {
        diesel::update(access_token.filter(id.eq(id_to_delete)))
            .filter(is_deleted.eq(false))
            .set(body)
            .returning(AccessTokenModel::as_returning())
            .get_result::<Self>(conn).map_err(|x| x.into())
    }

    pub fn get(conn: &mut PgConnection, id_to_find: String) -> Result<Self, DatabaseException> {
        access_token.select(Self::as_select())
            .filter(is_deleted.eq(false))
            .filter(id.eq(id_to_find))
            .first::<Self>(conn).map_err(|x| x.into())
    }
}