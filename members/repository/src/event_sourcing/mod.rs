use crate::repo::Repository;
use anyhow::Result;
use events::Event;

mod aggregate;
mod command;
mod event_store;
pub mod tweet;

pub fn store_events(repo: &mut Repository, events: &Vec<Event>) -> Result<()> {
    for event in events {
        let doc = event_store::InsertEvent::from(event.to_owned());
        event_store::save_event(repo, doc)?;
        message_broker::publish(event)?;
    }
    Ok(())
}
