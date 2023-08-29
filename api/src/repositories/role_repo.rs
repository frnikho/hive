use chrono::Utc;
use diesel::PgConnection;
use crate::entities::pagination::Pagination;
use crate::entities::role::{Role, RoleList};
use crate::entities::user::User;
use crate::exceptions::api::ApiException;
use crate::models::role::{CreateRoleModel, RoleModel, UpdateRoleModel};

pub struct RoleRepo;

#[derive(Clone, Debug)]
pub struct CreateRole {
    pub name: String,
    pub description: Option<String>,
    pub actions: Option<Vec<String>>,
    pub models: Option<Vec<String>>,
    pub triggers: Option<Vec<String>>,
    pub created_by_user_id: Option<String>,
}

impl Into<CreateRoleModel> for CreateRole {
    fn into(self) -> CreateRoleModel {
        CreateRoleModel {
            name: self.name,
            description: self.description,
            actions: self.actions,
            models: self.models,
            triggers: self.triggers,
            created_by_user_id: self.created_by_user_id,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct UpdateRole {
    pub name: Option<String>,
    pub description: Option<String>,
    pub actions: Option<Vec<String>>,
    pub models: Option<Vec<String>>,
    pub triggers: Option<Vec<String>>,
    pub updated_by_user_id: Option<String>,
}

impl Into<UpdateRoleModel> for UpdateRole {
    fn into(self) -> UpdateRoleModel {
        UpdateRoleModel {
            name: self.name,
            description: self.description,
            actions: self.actions,
            models: self.models,
            triggers: self.triggers,
            updated_by_user_id: self.updated_by_user_id,
            updated_date: Some(Utc::now().naive_utc()),
        }
    }
}

impl RoleRepo {
    pub fn list(conn: &mut PgConnection, pag: Pagination) -> Result<RoleList, ApiException> {
        RoleModel::list(conn, pag)
            .map_err(|x| x.into())
            .map(|x| Role::from_vec_model(x))
    }

    pub fn for_user(user: &User, conn: &mut PgConnection, pag: &Pagination) -> Result<RoleList, ApiException> {
        user.clone().into_model().list_roles(conn, pag)
            .map_err(|x| x.into())
            .map(|x| Role::from_vec_model(x))
    }
}