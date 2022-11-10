use diesel::{RunQueryDsl};
use model::{TweetData, Tweet};

use crate::{repo::{Repository}, postgres::{NewEvent}, schema};

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