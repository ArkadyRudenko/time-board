-- Your SQL goes here
alter table sessions alter column start_task set default CURRENT_TIMESTAMP;
alter table sessions alter column end_task set default CURRENT_TIMESTAMP;
