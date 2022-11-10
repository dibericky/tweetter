use std::ops::Deref;

use model::{TweetData, Tweet};
use serde::{Serialize, Deserialize};

use crate::{repo::{Repository, Table}, postgres::StoredEvent};

use self::event::Event;

pub mod event;

pub enum Command {
    AddTweet(TweetData)
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
        // let doc = StoredEvent::new(event.to_owned());
        // repo.add(Table::EVENTS, &doc);
    }
}