create table devices
(
    id                 varchar   default nanoid() not null
        constraint devices_pk
            primary key,
    name               varchar(255)               not null,
    created_by_user_id varchar
        constraint devices_users_id_fk
            references users
            on update cascade on delete set null,
    description        varchar(4096),
    created_date       timestamp default now()    not null,
    updated_date       timestamp,
    deleted_date       timestamp,
    is_deleted         boolean   default false    not null,
    is_activated       boolean   default false    not null,
    updated_by_user_id varchar
        constraint devices_users_id_fk2
            references users
            on update cascade on delete set null,
    deleted_by_user_id varchar
        constraint devices_users_id_fk3
            references users
            on update cascade on delete set null
);

create table devices_pull_history
(
    id                varchar   default nanoid() not null
        constraint devices_pull_history_pk
            primary key,
    pull_by_device_id varchar                    not null,
    pull_date         timestamp default now()    not null,
    pull_by_user_id   integer,
    status            varchar
);

create table devices_ping_history
(
    id varchar default nanoid() not null
);

create table devices_startup_history
(
    id varchar default nanoid() not null
);

create table devices_status
(
    id           varchar   default nanoid() not null
        constraint devices_status_pk
            primary key,
    device_id    varchar                    not null
        constraint devices_status_devices_id_fk
            references devices
            on update cascade on delete cascade,
    created_date timestamp default now(),
    status       varchar
);

