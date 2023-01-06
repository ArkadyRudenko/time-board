// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Text,
        user_id -> Uuid,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        task_id -> Uuid,
        start_task -> Timestamp,
        end_task -> Timestamp,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        description -> Text,
        project_id -> Uuid,
    }
}

diesel::table! {
    tokens (token) {
        token -> Varchar,
        user_id -> Uuid,
        created_at -> Timestamp,
        last_used_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        login -> Varchar,
        password -> Text,
    }
}

diesel::joinable!(projects -> users (user_id));
diesel::joinable!(sessions -> tasks (task_id));
diesel::joinable!(tasks -> projects (project_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    sessions,
    tasks,
    tokens,
    users,
);
