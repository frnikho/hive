-- Your SQL goes here
create table firmwares
(
    id varchar default nanoid() not null
        constraint firmwares_pk
            primary key
);