// @generated automatically by Diesel CLI.

diesel::table! {
    credentials (id) {
        id -> Varchar,
        user_id -> Varchar,
        email -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        code -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tasks (id) {
        id -> Varchar,
        title -> Varchar,
        description -> Nullable<Text>,
        estimated_time -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        last_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Varchar,
        user_id -> Varchar,
        role_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(credentials -> users (user_id));
diesel::joinable!(users_roles -> roles (role_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    credentials,
    roles,
    tasks,
    users,
    users_roles,
);
