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
    pub is_deleted: bool,
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
            is_deleted: model.is_deleted,
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
    
    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }

    pub fn into_model(self) -> UserModel {
        UserModel {
            id: self.id,
            email: self.email,
            firstname: self.firstname,
            lastname: self.lastname,
            password: self.password,
            created_date: self.created_date,
            updated_date: self.updated_date,
            deleted_date: self.deleted_date,
            updated_by_user_id: self.updated_by.as_ref().map(|x| x.id.clone()),
            deleted_by_user_id: self.deleted_by.as_ref().map(|x| x.id.clone()),
            created_by_user_id: self.created_by.as_ref().map(|x| x.id.clone()),
            is_deleted: self.is_deleted,
        }
    }

}

impl Into<User> for UserModel {
    fn into(self) -> User {
        User::new(&self)
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