use anyhow::Result;
use model::{Tweet, TweetMessage, TweetID};
use crate::{repo::{Repository}};
use events::Event;

mod event_store;
pub mod tweet;

pub enum Command {
    AddTweet(Tweet),
    EditTweet(TweetID, TweetMessage)
}


pub fn run_command(command: Command) -> Vec<Event> {
    match command {
        Command::AddTweet(data) => {
            vec![Event::TweetAdded(data)]
        },
        Command::EditTweet(id, msg) => {
            vec![Event::TweetMessageEdited((id, msg).into())]
        },
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