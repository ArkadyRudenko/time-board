-- This file should undo anything in `up.sql`

drop
index session_id_uindex;
drop table sessions;