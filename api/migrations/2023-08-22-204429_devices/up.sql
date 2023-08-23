-- Your SQL goes here
create table devices
(
    id varchar default nanoid() not null
        constraint devices_pk
            primary key
);