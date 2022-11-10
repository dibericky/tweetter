use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::Timestamptz;
use dotenvy::dotenv;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::env;
use crate::event_sourcing::event::Event;
use crate::schema::events;

pub fn get_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Queryable)]
pub struct StoredEvent {
    id: i32,
    payload: String,
    event_type: String,
    aggregate_id: String,
    created_at: Timestamptz
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub payload: String,
    pub event_type: String,
    pub aggregate_id: String
}

impl NewEvent {
    pub fn new(event: &Event) -> Self {
        Self {
            payload: serde_json::to_string(&event).unwrap(),
            event_type: event.event_type(),
            aggregate_id: event.aggregate_id()
        }
    }
}