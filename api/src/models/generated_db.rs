// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role_type"))]
    pub struct RoleType;
}

diesel::table! {
    devices (id) {
        id -> Varchar,
    }
}

diesel::table! {
    firmwares (id) {
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 4096]
        description -> Nullable<Varchar>,
        created_date -> Timestamp,
        updated_date -> Nullable<Timestamp>,
        deleted_date -> Nullable<Timestamp>,
        created_by_user_id -> Nullable<Varchar>,
        updated_by_user_id -> Nullable<Varchar>,
        deleted_by_user_id -> Nullable<Varchar>,
        is_deleted -> Bool,
        is_activated -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RoleType;

    roles (id) {
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 4096]
        description -> Nullable<Varchar>,
        created_by_user_id -> Nullable<Varchar>,
        is_activated -> Bool,
        is_deleted -> Bool,
        created_date -> Timestamp,
        deleted_date -> Nullable<Timestamp>,
        actions -> Array<Nullable<Text>>,
        models -> Array<Nullable<Text>>,
        triggers -> Array<Nullable<Text>>,
        role_type -> RoleType,
        #[max_length = 255]
        key -> Nullable<Varchar>,
        updated_date -> Nullable<Timestamp>,
        updated_by_user_id -> Nullable<Varchar>,
        deleted_by_user_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        #[max_length = 512]
        email -> Varchar,
        #[max_length = 512]
        password -> Varchar,
        #[max_length = 255]
        firstname -> Varchar,
        #[max_length = 255]
        lastname -> Varchar,
        created_date -> Timestamp,
        updated_date -> Nullable<Timestamp>,
        deleted_date -> Nullable<Timestamp>,
        created_by_user_id -> Nullable<Varchar>,
        updated_by_user_id -> Nullable<Varchar>,
        deleted_by_user_id -> Nullable<Varchar>,
        is_deleted -> Bool,
    }
}

diesel::table! {
    users_roles (user_id, role_id) {
        id -> Varchar,
        user_id -> Varchar,
        role_id -> Varchar,
        created_date -> Timestamp,
        created_by -> Nullable<Varchar>,
    }
}

diesel::joinable!(users_roles -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    firmwares,
    roles,
    users,
    users_roles,
);
