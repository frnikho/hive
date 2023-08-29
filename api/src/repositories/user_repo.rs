use chrono::NaiveDateTime;
use diesel::PgConnection;
use crate::entities::pagination::Pagination;
use crate::entities::user::{User, UserList};
use crate::exceptions::api::ApiException;
use crate::models::user::{CreateUserModel, UpdateUserModel, UserModel};

pub struct UserRepository;

#[derive(Clone, Debug)]
pub struct UpdateUser {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by_user_id: Option<String>,
}

impl Into<UpdateUserModel> for UpdateUser {
    fn into(self) -> UpdateUserModel {
        UpdateUserModel {
            firstname: self.firstname,
            lastname: self.lastname,
            updated_date: self.updated_date,
            updated_by_user_id: self.updated_by_user_id,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateUser {
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub created_by_user_id: Option<String>,
}

impl Into<CreateUserModel> for CreateUser {
    fn into(self) -> CreateUserModel {
        CreateUserModel {
            email: self.email,
            firstname: self.firstname,
            lastname: self.lastname,
            password: self.password,
            created_by_user_id: self.created_by_user_id,
        }
    }
}

pub enum UserFindOneClause {
    Id(String),
    Email(String),
}

pub enum UserFindManyClause {
    Firstname(String),
    Lastname(String),
    IsActivated(bool),
}

impl UserRepository {

    pub fn find(conn: &mut PgConnection, clause: &UserFindOneClause) -> Result<User, ApiException> {
        match clause {
            UserFindOneClause::Id(x) => UserModel::find(conn, x),
            UserFindOneClause::Email(x) => UserModel::find_by_email(conn, x),
        }
            .map(|x|x.into())
            .map_err(|x| x.into())
    }

    pub fn list(conn: &mut PgConnection, pag: &Pagination) -> Result<UserList, ApiException> {
        UserModel::list(conn, pag)
            .map(|x|x.into_iter().map(|x|x.into()).collect())
            .map_err(|x| x.into())
    }

    pub fn delete(conn: &mut PgConnection, id_to_delete: &String, deleted_by: &Option<String>) -> Result<User, ApiException> {
        UserModel::delete(conn, id_to_delete, deleted_by)
            .map(|x|x.into())
            .map_err(|x| x.into())
    }

    pub fn update(conn: &mut PgConnection, id_to_update: &String, update_user: UpdateUser) -> Result<User, ApiException> {
        UserModel::update(conn, id_to_update, &update_user.into())
            .map(|x|x.into())
            .map_err(|x| x.into())
    }

    pub fn create(conn: &mut PgConnection, create_user: CreateUser) -> Result<User, ApiException> {
        UserModel::create(conn, &create_user.into())
            .map(|x|x.into())
            .map_err(|x| x.into())
    }
}