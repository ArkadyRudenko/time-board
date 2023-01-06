-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table tasks
(
    id             uuid default uuid_generate_v4() not null
            primary key,
    description    text                            not null,
    project_id uuid not null
            constraint tasks_to_project
                references projects
                on delete cascade
);

create unique index IF NOT EXISTS tasks_id_uindex
    on tasks (id);
