use chrono::NaiveDateTime;

pub type RoleList = Vec<Role>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub created_date: NaiveDateTime,
}