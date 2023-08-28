use chrono::NaiveDateTime;
use crate::entities::user::User;
use crate::models::role::RoleModel;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_date: NaiveDateTime,
    pub created_by: Option<Box<User>>,
    pub updated_date: Option<NaiveDateTime>,
    pub deleted_date: Option<NaiveDateTime>,
    pub deleted_by: Option<Box<User>>,
    pub updated_by: Option<Box<User>>,
    pub is_activated: bool,
    pub is_deleted: bool,
    pub actions: Vec<Option<String>>,
    pub models: Vec<Option<String>>,
    pub triggers: Vec<Option<String>>,
}

impl Into<Role> for RoleModel {
    fn into(self) -> Role {
        Role {
            id: self.id,
            name: self.name,
            description: self.description,
            created_date: self.created_date,
            created_by: None,
            deleted_by: None,
            updated_by: None,
            updated_date: self.updated_date,
            deleted_date: self.deleted_date,
            is_activated: self.is_activated,
            is_deleted: self.is_deleted,
            actions: self.actions,
            models: self.models,
            triggers: self.triggers,
        }
    }
}
pub type RoleList = Vec<Role>;

impl Role {

    pub fn from_vec_model(models: Vec<RoleModel>) -> RoleList {
        models.iter().map(|x| x.clone().into()).collect()
    }

    pub fn set_created(&mut self, user: User) -> &mut Self {
        self.created_by = Some(Box::new(user));
        self
    }

    pub fn set_updated(&mut self, user: User) -> &mut Self {
        self.updated_by = Some(Box::new(user));
        self
    }

    pub fn set_deleted(&mut self, user: User) -> &mut Self {
        self.deleted_by = Some(Box::new(user));
        self
    }
}