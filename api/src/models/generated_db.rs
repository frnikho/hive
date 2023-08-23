// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Varchar,
    }
}

diesel::table! {
    firmwares (id) {
        id -> Varchar,
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

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    firmwares,
    users,
);
