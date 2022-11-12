// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        payload -> Varchar,
        event_type -> Varchar,
        aggregate_id -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tweets (id) {
        id -> Varchar,
        author -> Varchar,
        message -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(events, tweets,);
