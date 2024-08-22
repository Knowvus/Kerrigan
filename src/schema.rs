// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_users (id) {
        id -> Int4,
        email -> Varchar,
    }
}

diesel::joinable!(tasks -> user_users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    user_users,
);
