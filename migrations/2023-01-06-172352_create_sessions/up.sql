-- Your SQL goes here
create table sessions
(
    id             uuid default uuid_generate_v4() not null
                primary key,
    task_id uuid not null
                constraint session_to_task
                    references tasks
                    on delete cascade,
    start_task timestamp not null,
    end_task timestamp not null
);

create unique index session_id_uindex
	on sessions (id);
