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


create table devices_status
(
    id           varchar   default nanoid() not null
        constraint devices_status_pk
            primary key,
    device_id    varchar                    not null
        constraint devices_status_devices_id_fk
            references devices
            on update cascade on delete cascade,
    created_date timestamp default now()    not null,
    status       varchar                    not null
);


create table devices_history
(
    id                 varchar   default nanoid()                    not null
        constraint devices_history_pk
            primary key,
    operation          varchar   default 'UPDATE'::character varying not null,
    parent_id          varchar                                       not null
        constraint devices_history_devices_id_fk
            references devices
            on update cascade on delete cascade,
    old                jsonb     default '{}'::jsonb                 not null,
    new                jsonb     default '{}'::jsonb                 not null,
    updated_date       timestamp default now()                       not null,
    updated_by_user_id varchar
        constraint devices_history_users_id_fk
            references users
            on update cascade on delete set null
);


CREATE OR REPLACE FUNCTION history_update_device() RETURNS TRIGGER AS $device_history$
BEGIN
    INSERT INTO devices_history(operation, parent_id, old, new, updated_date, updated_by_user_id)  VALUES('UPDATE', NEW."id", row_to_json(OLD)::jsonb, row_to_json(NEW)::jsonb, now(), NEW."updated_by_user_id");
    RETURN NEW;
END;
$device_history$ LANGUAGE plpgsql;

CREATE TRIGGER device_before_update BEFORE UPDATE ON devices FOR EACH ROW EXECUTE PROCEDURE history_update_device();