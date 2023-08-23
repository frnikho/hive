-- Your SQL goes here
create type role_type as enum ('system', 'custom');

create table roles
(
    id                 varchar   default nanoid()            not null
        constraint roles_pk
            primary key,
    name               varchar(255)                          not null,
    description        varchar(4096),
    created_by_user_id varchar
        constraint roles_created_users_id_fk
            references users
            on update cascade on delete set null,
    is_activated       boolean   default true                not null,
    is_deleted         boolean   default false               not null,
    created_date       timestamp default now()               not null,
    deleted_date       timestamp,
    actions            text[]    default '{}'::text[]        not null,
    models             text[]    default '{}'::text[]        not null,
    triggers           text[]    default '{}'::text[]        not null,
    role_type          role_type default 'custom'::role_type not null,
    key                varchar(255),
    updated_date       timestamp,
    updated_by_user_id varchar
        constraint roles_updated_users_id_fk
            references users
            on update cascade on delete set null,
    deleted_by_user_id varchar
        constraint roles_deleted_users_id_fk
            references users
            on update cascade on delete set null
);

create table users_roles
(
    id           varchar   default nanoid() not null,
    user_id      varchar                    not null
        constraint users_roles_users_id_fk
            references users
            on update cascade on delete cascade,
    role_id      varchar                    not null
        constraint users_roles_roles_id_fk
            references roles
            on update cascade on delete cascade,
    created_date timestamp default now()    not null,
    created_by   varchar
        constraint users_roles_users_id_fk2
            references users
            on update cascade on delete set null,
    constraint users_roles_pk
        primary key (user_id, role_id)
);

--- Insert default roles (Default & Admin)
INSERT INTO public.roles (id, name, description, created_by_user_id, is_activated, is_deleted, created_date, deleted_date, actions, models, triggers, role_type, key, updated_date, updated_by_user_id, deleted_by_user_id) VALUES (DEFAULT, 'Default'::varchar(255), 'Default user role'::varchar(4096), null::varchar, true::boolean, false::boolean, DEFAULT, null::timestamp, '{}', '{}', '{}', 'system'::role_type, 'default'::varchar(255), null::timestamp, null::varchar, null::varchar);
INSERT INTO public.roles (id, name, description, created_by_user_id, is_activated, is_deleted, created_date, deleted_date, actions, models, triggers, role_type, key, updated_date, updated_by_user_id, deleted_by_user_id) VALUES (DEFAULT, 'Admin'::varchar(255), 'Default admin role'::varchar(4096), null::varchar, true::boolean, false::boolean, DEFAULT, null::timestamp, '{*}', '{*}', '{*}', 'system'::role_type, 'super_user'::varchar(255), null::timestamp, null::varchar, null::varchar);