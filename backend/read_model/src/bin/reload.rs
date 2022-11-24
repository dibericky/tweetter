use std::sync::{Arc, Mutex};

use read_model::event_handler;
use repository::{events::get_all_events, repo::Repository};

fn main() {
    let mut repo = Repository::default();
    let all_events = get_all_events(&mut repo).expect("failed to load all events");
    println!("Events retrieved from event store");
    let mut repo = Arc::new(Mutex::new(repo));
    println!("Starting to project events");
    for event in &all_events {
        event_handler(&mut repo, event).expect("Failed to handle event");
        println!("Event projected correctly");
    }
    println!("All events have been projected");
}
