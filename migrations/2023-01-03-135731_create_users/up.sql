-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


create table users
(
    id       uuid default uuid_generate_v4() not null
        constraint table_name_pk
            primary key,
    username varchar(128)                    not null,
    login    varchar(128)                    not null,
    password   text                          not null
);

create unique index IF NOT EXISTS users_id_uindex
    on users (id);

create unique index IF NOT EXISTS users_username_uindex
    on users (username);

create unique index IF NOT EXISTS users_login_uindex
    on users (login);