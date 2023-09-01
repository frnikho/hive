use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::entities::pagination::Pagination;
use crate::exceptions::db::DatabaseException;
use crate::models::generated_db::users::dsl::*;
use crate::models::role::RoleModel;
use crate::models::access_token::AccessTokenModel;
use crate::models::generated_db::users_access_token;
use crate::models::generated_db::users_roles;

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

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::models::generated_db::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserIdModel {
    pub id: String,
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

impl Default for UpdateUserModel {
    fn default() -> Self {
        Self {
            email: None,
            firstname: None,
            lastname: None,
            password: None,
            is_deleted: None,
            deleted_date: None,
            updated_date: None,
            updated_by_user_id: None,
            deleted_by_user_id: None,
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[diesel(belongs_to(UserModel, foreign_key = user_id))]
#[diesel(belongs_to(crate::models::role::RoleModel, foreign_key = role_id))]
#[diesel(table_name = crate::models::generated_db::users_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id, role_id))]
pub struct UserRoleModel {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub created_date: NaiveDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[diesel(belongs_to(UserModel, foreign_key = user_id))]
#[diesel(belongs_to(AccessTokenModel, foreign_key = access_token_id))]
#[diesel(table_name = crate::models::generated_db::users_access_token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(user_id, access_token_id))]
pub struct UserAccessTokenModel {
    pub user_id: String,
    pub access_token_id: String,
}

impl UserModel {

    pub fn create(conn: &mut PgConnection, new_user: &CreateUserModel) -> Result<Self, DatabaseException> {
        diesel::insert_into(users)
            .values(new_user)
            .returning(UserModel::as_returning())
            .get_result::<UserModel>(conn).map_err(|x| x.into())
    }

    pub fn find(conn: &mut PgConnection, id_to_find: &String) -> Result<Self, DatabaseException> {
        users.select(UserModel::as_select())
            .filter(is_deleted.eq(false))
            .filter(id.eq(id_to_find))
            .first::<UserModel>(conn).map_err(|x| x.into())
    }

    pub fn find_by_email(conn: &mut PgConnection, email_to_find: &String) -> Result<Self, DatabaseException> {
        users.select(UserModel::as_select())
            .filter(is_deleted.eq(false))
            .filter(email.eq(email_to_find))
            .first::<UserModel>(conn).map_err(|x| x.into())
    }

    pub fn list(conn: &mut PgConnection, pag: &Pagination) -> Result<Vec<Self>, DatabaseException> {
        let user = users.select(UserModel::as_select()).filter(is_deleted.eq(false)).into_boxed();
        match pag.bypass {
            true => user,
            false => user.offset((pag.page * pag.limit) as i64).limit(pag.limit as i64),
        }.load::<UserModel>(conn).map_err(|x| x.into())
    }

    pub fn delete(conn: &mut PgConnection, id_to_delete: &String, deleted_by: &Option<String>) -> Result<Self, DatabaseException> {
        diesel::update(users.filter(id.eq(id_to_delete)))
            .filter(is_deleted.eq(false))
            .set((is_deleted.eq(true), deleted_date.eq(Utc::now().naive_utc()), deleted_by_user_id.eq(deleted_by)))
            .get_result::<UserModel>(conn).map_err(|x| x.into())
    }

    pub fn update(conn: &mut PgConnection, id_to_update: &String, update_user: &UpdateUserModel) -> Result<Self, DatabaseException> {
        diesel::update(users.filter(id.eq(id_to_update)))
            .filter(is_deleted.eq(false))
            .set(update_user)
            .get_result::<UserModel>(conn).map_err(|x| x.into())
    }

    pub fn list_roles(&self, conn: &mut PgConnection, pag: &Pagination) -> Result<Vec<RoleModel>, DatabaseException> {
        let query = UserRoleModel::belonging_to(&self)
            .inner_join(crate::models::generated_db::roles::dsl::roles).into_boxed();
        match pag.bypass {
            true => query,
            false => query.limit(pag.limit as i64).offset((pag.page * pag.limit) as i64)
        }.select(RoleModel::as_select())
            .load::<RoleModel>(conn)
            .map_err(|err| err.into())
    }

    pub fn get_role(&self, conn: &mut PgConnection, role_id_to_find: &String) -> Result<UserRoleModel, DatabaseException> {
        UserRoleModel::belonging_to(self)
            .inner_join(crate::models::generated_db::roles::table)
            .filter(crate::models::generated_db::roles::id.eq(role_id_to_find))
            .select(UserRoleModel::as_select())
            .first(conn).map_err(|x| x.into())
    }

    pub fn add_role(&self, conn: &mut PgConnection, role: &RoleModel) -> Result<UserRoleModel, DatabaseException> {
        diesel::insert_into(users_roles::table)
            .values((users_roles::user_id.eq(self.id.clone()), users_roles::role_id.eq(role.id.clone())))
            .returning(UserRoleModel::as_returning())
            .get_result(conn).map_err(|x|x.into())
    }

    pub fn remove_role(&self, conn: &mut PgConnection, role: &RoleModel) -> Result<(), DatabaseException> {
        self.get_role(conn, &role.id.clone())?;
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(self.id.clone()).and(users_roles::role_id.eq(role.id.clone()))))
            .execute(conn)
            .map(|_| ())
            .map_err(|x| x.into())
    }

    pub fn get_access_token(&self, conn: &mut PgConnection, token_id: &String) -> Result<UserAccessTokenModel, DatabaseException> {
        UserAccessTokenModel::belonging_to(self)
            .inner_join(crate::models::generated_db::access_token::table)
            .filter(crate::models::generated_db::access_token::id.eq(token_id))
            .select(UserAccessTokenModel::as_select())
            .first(conn).map_err(|x| x.into())
    }

    pub fn list_access_token(&self, conn: &mut PgConnection, pag: &Pagination) -> Result<Vec<AccessTokenModel>, DatabaseException> {
        let query = UserAccessTokenModel::belonging_to(&self)
            .inner_join(crate::models::generated_db::access_token::table).into_boxed();
        match pag.bypass {
            true => query,
            false => query.limit(pag.limit as i64).offset((pag.page * pag.limit) as i64)
        }.select(AccessTokenModel::as_select())
            .load::<AccessTokenModel>(conn)
            .map_err(|err| err.into())
    }

    pub fn add_access_token(&self, conn: &mut PgConnection, token_id: &String) -> Result<UserAccessTokenModel, DatabaseException> {
        diesel::insert_into(users_access_token::table)
            .values((users_access_token::user_id.eq(self.id.clone()), users_access_token::access_token_id.eq(token_id.clone())))
            .returning(UserAccessTokenModel::as_returning())
            .get_result(conn).map_err(|x|x.into())
    }

    pub fn remove_access_token(&self, conn: &mut PgConnection, token_id: &String) -> Result<(), DatabaseException> {
        self.get_access_token(conn, token_id)?;
        diesel::delete(users_access_token::table.filter(users_access_token::user_id.eq(self.id.clone()).and(users_access_token::access_token_id.eq(token_id.clone()))))
            .execute(conn)
            .map(|_| ())
            .map_err(|x| x.into())
    }

}