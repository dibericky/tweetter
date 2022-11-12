use crate::{schema, repo::Repository};
use anyhow::Result;
use diesel::{RunQueryDsl, Insertable};
use events::Event;
use serde::{Serialize, Deserialize};
use crate::schema::events as EventsSchema;

pub fn save_event(repo: &mut Repository, doc: InsertEvent) -> Result<usize> {
    diesel::insert_into(schema::events::table)
            .values(&doc)
            .execute(repo.get_connection())
            .map_err(|_| anyhow::anyhow!("Failed to store event"))
}

// #[derive(Queryable)]
// pub struct StoredEvent {
//     id: i32,
//     payload: String,
//     event_type: String,
//     aggregate_id: String,
//     created_at: Timestamptz
// }

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = EventsSchema)]
pub struct InsertEvent {
    pub payload: String,
    pub event_type: String,
    pub aggregate_id: String
}

impl From<Event> for InsertEvent {
    fn from(event: Event) -> Self {
        Self { 
            payload: serde_json::to_string(&event).unwrap(),
            event_type: event.event_type(),
            aggregate_id: event.aggregate_id()
        }
    }
}