-- Your SQL goes here
create table users
(
    id                 varchar   default nanoid() not null
        constraint users_id_pk
            primary key,
    email              varchar(512)               not null
        constraint users_email_pk
            unique,
    password           varchar(512)               not null,
    firstname          varchar(255)               not null,
    lastname           varchar(255)               not null,
    created_date       timestamp default now()    not null,
    updated_date       timestamp,
    deleted_date       timestamp,
    created_by_user_id varchar
        constraint users_created_users_id_fk
            references users
            on update cascade on delete set null,
    updated_by_user_id varchar
        constraint users_updated_users_id_fk
            references users
            on update cascade on delete set null,
    deleted_by_user_id varchar
        constraint users_deleted_users_id_fk
            references users
            on update cascade on delete set null,
    is_deleted         boolean   default false    not null
);