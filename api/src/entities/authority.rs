
use crate::entities::role::RoleList;
use crate::entities::user::User;



pub enum Authority {
    User(User, RoleList),
    AccessToken(User, RoleList),
}

impl Authority {
    pub fn get_user(&self) -> &User {
        match self {
            Authority::User(user, ..) => user,
            Authority::AccessToken(user, ..) => user,
        }
    }
}

