use anyhow::Result;
use events::Event;

use crate::{event_sourcing::event_store, repo::Repository};

pub trait Aggregate {
    type State;
    type Command;
    type Event;

    fn aggregate_type() -> &'static str;

    fn initial_state() -> Self::State;

    fn load(repo: &mut Repository, id: &str) -> Result<Option<Self::State>> {
        let events = event_store::get_events_by(repo, id, Self::aggregate_type())?;

        if events.is_empty() {
            return Ok(None);
        }
        let state = events
            .into_iter()
            .fold(Self::initial_state(), |acc, event| {
                Self::handle_event(acc, &event)
            });
        Ok(Some(state))
    }

    fn handle_event(state: Self::State, event: &Event) -> Self::State;

    fn run_command(state: Option<Self::State>, command: Self::Command) -> Vec<Self::Event>;
}
