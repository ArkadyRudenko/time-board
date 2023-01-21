-- This file should undo anything in `up.sql`

alter table sessions alter column start_task set default null;
alter table sessions alter column end_task set default null;