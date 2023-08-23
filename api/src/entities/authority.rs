use crate::entities::user::User;

pub enum Authority {
    User(User),
}