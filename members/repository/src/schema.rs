// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        payload -> Varchar,
        event_type -> Varchar,
        aggregate_id -> Varchar,
        aggregate_type -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tweets (id) {
        id -> Varchar,
        author_id -> Varchar,
        message -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_profile (id) {
        id -> Varchar,
        nickname -> Varchar,
        num_tweets -> Int4,
        following -> Int4,
        follower -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(events, tweets, user_profile,);
