use chrono::NaiveDateTime;
use crate::models::user::UserModel;

pub type UserList = Vec<User>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub created_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>,
    pub deleted_date: Option<NaiveDateTime>,
    pub updated_by: Option<Box<User>>,
    pub deleted_by: Option<Box<User>>,
    pub created_by: Option<Box<User>>,
}

impl User {
    pub fn new(model: &UserModel) -> Self {
        User {
            id: model.id.clone(),
            email: model.email.clone(),
            firstname: model.firstname.clone(),
            lastname: model.lastname.clone(),
            password: model.password.clone(),
            created_date: model.created_date.clone(),
            updated_date: model.updated_date.clone(),
            deleted_date: model.deleted_date.clone(),
            updated_by: None,
            deleted_by: None,
            created_by: None,
        }
    }

    pub fn set_updated_by(&mut self, user: &User) -> &mut Self {
        self.updated_by = Some(Box::new(user.clone()));
        self
    }

    pub fn set_deleted_by(&mut self, user: &User) -> &mut Self {
        self.deleted_by = Some(Box::new(user.clone()));
        self
    }

    pub fn set_created_by(&mut self, user: &User) -> &mut Self {
        self.created_by = Some(Box::new(user.clone()));
        self
    }

}

#[cfg(test)]
mod test {
    use crate::entities::user::User;
    use crate::models::user::UserModel;

    #[test]
    pub fn test_create_user_model() {
        let mut user = User::new(&UserModel::default());
        let created_user = User::new(&UserModel::default());
        let user = user
            .set_created_by(&created_user)
            .set_updated_by(&created_user);
    }

}