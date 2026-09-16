// @generated automatically by Diesel CLI.

diesel::table! {
    achievements (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        points -> Int4,
        #[max_length = 50]
        status -> Varchar,
        created_by_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    action_logs (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 100]
        action -> Varchar,
        details -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    profiles (id) {
        id -> Int4,
        user_id -> Int4,
        bio -> Text,
        #[max_length = 255]
        avatar_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_achievements (id) {
        id -> Int4,
        user_id -> Int4,
        achievement_id -> Int4,
        obtained_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        // unique
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 20]
        role -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(achievements -> users (created_by_id));
diesel::joinable!(action_logs -> users (user_id));
diesel::joinable!(profiles -> users (user_id));
diesel::joinable!(user_achievements -> achievements (achievement_id));
diesel::joinable!(user_achievements -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    achievements,
    action_logs,
    profiles,
    user_achievements,
    users,
);
