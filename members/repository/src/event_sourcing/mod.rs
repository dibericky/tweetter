use crate::repo::Repository;
use anyhow::Result;
use events::{Event, TweetAddedPayload, TweetMessageEditedPayload};

mod aggregate;
mod event_store;
pub mod tweet;

pub struct AddTweetPayload {
    id: String,
    author_id: String,
    message: String,
}

impl AddTweetPayload {
    pub fn new(id: String, message: String, author_id: String) -> Self {
        Self {
            id,
            author_id,
            message,
        }
    }
}

pub struct EditTweetMessagePayload {
    id: String,
    author_id: String,
    message: String,
}

impl EditTweetMessagePayload {
    pub fn new(id: &str, author_id: &str, message: &str) -> Self {
        Self {
            message: message.to_owned(),
            author_id: author_id.to_owned(),
            id: id.to_owned(),
        }
    }
}

pub enum Command {
    AddTweet(AddTweetPayload),
    EditTweetMessage(EditTweetMessagePayload),
}

pub fn run_command(command: Command) -> Vec<Event> {
    match command {
        Command::AddTweet(data) => {
            let event = TweetAddedPayload::new(data.id, data.message, data.author_id);
            vec![Event::TweetAdded(event)]
        }
        Command::EditTweetMessage(data) => {
            let event = TweetMessageEditedPayload::new(&data.id, &data.author_id, &data.message);
            vec![Event::TweetMessageEdited(event)]
        }
    }
}

pub fn store_events(repo: &mut Repository, events: &Vec<Event>) -> Result<()> {
    for event in events {
        let doc = event_store::InsertEvent::from(event.to_owned());
        event_store::save_event(repo, doc)?;
        message_broker::publish(event)?;
    }
    Ok(())
}
