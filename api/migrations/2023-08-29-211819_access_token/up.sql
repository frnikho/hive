-- Your SQL goes here
create table access_token
(
    id                 varchar   default nanoid() not null
        constraint access_token_pk
            primary key,
    name               varchar(255)               not null,
    key                varchar(512)               not null,
    created_date       timestamp default now()    not null,
    deleted_date       timestamp,
    is_deleted         boolean   default false    not null,
    created_by_user_id varchar                    not null
        constraint access_token_users_id_fk
            references users,
    expiration         timestamp
);

create table users_access_token
(
    access_token_id varchar not null
        constraint users_access_token_access_token_id_fk
            references access_token
            on update cascade on delete cascade,
    user_id         varchar not null
        constraint users_access_token_users_id_fk
            references users
            on update cascade on delete cascade,
    constraint users_access_token_pk
        primary key (user_id, access_token_id)
);

