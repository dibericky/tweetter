use std::sync::{Arc, Mutex};

use anyhow::Result;
use events::Event;
use repository::{
    read_models::{
        self,
        tweets::{Tweet, UpdateTweet},
    },
    repo::Repository,
};

pub fn update(repo: &mut Arc<Mutex<Repository>>, event: &Event) -> Result<()> {
    match event {
        Event::TweetAdded(payload) => {
            let doc = Tweet::from(payload);
            read_models::tweets::insert(repo, doc)?;
            // read_models::user_profile::update(repo, &payload.author_id, doc)?;
        }
        Event::TweetMessageEdited(payload) => {
            let doc = UpdateTweet::from(payload);
            read_models::tweets::update(repo, &payload.id, doc)?;
        }
        Event::UserProfileAdded(_) => todo!(),
        Event::UserProfileEdited(_) => todo!(),
    };
    Ok(())
}
