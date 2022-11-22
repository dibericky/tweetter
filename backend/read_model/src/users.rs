use std::sync::{Arc, Mutex};

use anyhow::Result;
use events::Event;
use repository::{
    read_models::user_profile::{self, UpdateUserProfile, UserProfile},
    repo::Repository,
};

pub fn update(repo: &mut Arc<Mutex<Repository>>, event: &Event) -> Result<()> {
    match event {
        Event::UserTweetAdded(_) => {}
        Event::UserTweetMessageEdited(_) => {}
        Event::UserCreated(payload) => {
            let user = UserProfile::from(payload.to_owned());
            user_profile::insert(repo, user)?;
        }
        Event::UserEdited(_) => {}
        Event::UserNumberTweetIncremented(payload) => {
            let mut changeset = UpdateUserProfile::default();
            changeset.set_num_tweets(payload.num_tweet);

            user_profile::update(repo, &payload.id, changeset)?;
        }
    };
    Ok(())
}
