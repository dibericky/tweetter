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
