use std::sync::{Arc, Mutex};

use anyhow::Result;
use repository::{self, read_models::tweets::Tweet, repo::Repository};
use uuid::Uuid;

pub fn add_tweet(repo: &mut Repository, author_id: &str, msg: &str) -> Result<String> {
    println!("{} is tweeting {:?}", author_id, msg);
    let tweet_id = Uuid::new_v4().to_string();
    repository::events::tweet::new(repo, tweet_id.to_owned(), author_id.to_owned(), msg.to_owned())?;
    println!("New tweet! {}", &tweet_id);
    Ok(tweet_id)
}

pub fn edit_tweet(repo: &mut Repository, id: &str, author_id: &str, msg: &str) -> Result<()> {
    repository::events::tweet::edit(repo, id, author_id, msg)?;
    println!("Edited tweet!");
    Ok(())
}

pub fn get_by_id(repo: &mut Arc<Mutex<Repository>>, id: &str) -> Result<Tweet> {
    println!("Get tweet with id {id}");
    repository::read_models::tweets::get_by_id(repo, id)
}
