use diesel::{RunQueryDsl};
use model::{Tweet, TweetMessage, TweetID};

use crate::{repo::{Repository}, postgres::{NewEvent}, schema};

use self::event::Event;

pub mod event;

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

pub fn store_events(repo: &mut Repository, events: &Vec<Event>) {
    for event in events {
        // let doc = StoredEvent::new(event.to_owned());
        let doc = NewEvent::new(event);
        diesel::insert_into(schema::events::table)
            .values(&doc)
            .execute(repo.get_connection())
            .expect("Error saving new post");
    }
}