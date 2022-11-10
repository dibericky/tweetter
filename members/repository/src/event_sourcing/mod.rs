use std::ops::Deref;

use model::{TweetData, Tweet};
use serde::{Serialize, Deserialize};

use crate::repo::{Repository, Table};

pub enum Command {
    AddTweet(TweetData)
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag="type")]
pub enum Event {
    TweetAdded(Tweet)
}

#[derive(Serialize, Deserialize)]
struct StoredEvent {
    payload: Event
}
impl StoredEvent {
    pub fn new(payload: Event)->Self {
        Self { payload }
    }
}

pub fn run_command(command: Command) -> Vec<Event> {
    match command {
        Command::AddTweet(data) => {
            let id = uuid::Uuid::new_v4();
            let id = id.to_string();
            let tweet : Tweet = (id, data).into();
            vec![Event::TweetAdded(tweet)]
        },
    }
}

pub fn store_events(repo: &Repository, events: &Vec<Event>) {
    for event in events {
        let doc = StoredEvent::new(event.to_owned());
        repo.add(Table::EVENTS, &doc);
    }
}