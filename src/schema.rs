// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        name -> Varchar,
        last_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}
