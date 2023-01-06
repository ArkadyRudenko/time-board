-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


create table projects
(
    id             uuid default uuid_generate_v4() not null
            primary key,
    title          varchar(128)                    not null,
    description    text                            not null,
    user_id uuid not null
            constraint project_to_users
                references users
                on delete cascade
);

create unique index IF NOT EXISTS projects_id_uindex
    on projects (id);
