use crate::schema::events as EventsSchema;
use crate::{repo::Repository, schema};
use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::ExpressionMethods;
use diesel::{Insertable, QueryDsl, Queryable, RunQueryDsl};
use events::Event;
use schema::events::dsl as Dsl;
use serde::{Deserialize, Serialize};

pub fn save_event(repo: &mut Repository, doc: InsertEvent) -> Result<usize> {
    diesel::insert_into(schema::events::table)
        .values(&doc)
        .execute(repo.get_connection())
        .map_err(|_| anyhow::anyhow!("Failed to store event"))
}

pub fn get_events_by(
    repo: &mut Repository,
    aggregate_id: &str,
    aggregate_type: &str,
) -> Result<Vec<Event>> {
    let row_events: Vec<StoredEvent> = schema::events::dsl::events
        .filter(Dsl::aggregate_id.eq(aggregate_id))
        .filter(Dsl::aggregate_type.eq(aggregate_type))
        .get_results(repo.get_connection())
        .map_err(|_| anyhow::anyhow!("Failed to store event"))?;

    Ok(row_events
        .into_iter()
        .map(|row| serde_json::from_str::<Event>(&row.payload).unwrap())
        .collect::<Vec<_>>())
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = EventsSchema)]
pub struct StoredEvent {
    pub id: i32,
    pub payload: String,
    pub event_type: String,
    pub aggregate_id: String,
    pub aggregate_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = EventsSchema)]
pub struct InsertEvent {
    pub payload: String,
    pub event_type: String,
    pub aggregate_id: String,
    pub aggregate_type: String,
}

impl From<Event> for InsertEvent {
    fn from(event: Event) -> Self {
        Self {
            payload: serde_json::to_string(&event).unwrap(),
            event_type: event.event_type(),
            aggregate_id: event.aggregate_id(),
            aggregate_type: event.aggregate_type(),
        }
    }
}
