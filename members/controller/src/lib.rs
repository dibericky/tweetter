use std::sync::{Arc, Mutex};

use anyhow::Result;
use repository::{self, read_models::{tweets::Tweet, user_profile::UserProfile}, repo::Repository};
use uuid::Uuid;

pub fn add_tweet(repo: &mut Repository, author_id: &str, msg: &str) -> Result<String> {
    println!("{} is tweeting {:?}", author_id, msg);
    let tweet_id = Uuid::new_v4().to_string();
    repository::events::tweet::new(
        repo,
        tweet_id.to_owned(),
        author_id.to_owned(),
        msg.to_owned(),
    )?;
    println!("New tweet! {tweet_id}");
    Ok(tweet_id)
}

pub fn edit_tweet(repo: &mut Repository, id: &str, author_id: &str, msg: &str) -> Result<()> {
    repository::events::tweet::edit(repo, id, author_id, msg)?;
    println!("Edited tweet!");
    Ok(())
}

pub fn create_user(repo: &mut Repository, nickname: &str) -> Result<String> {
    let user_id = Uuid::new_v4().to_string();
    repository::events::user::new(repo, &user_id, nickname)?;
    println!("User created! {user_id}");
    Ok(user_id)
}

pub fn get_tweet(repo: &mut Arc<Mutex<Repository>>, id: &str) -> Result<Tweet> {
    println!("Get tweet with id {id}");
    repository::read_models::tweets::get_by_id(repo, id)
}

pub fn get_user(repo: &mut Arc<Mutex<Repository>>, id: &str) -> Result<UserProfile> {
    println!("Get user with id {id}");
    repository::read_models::user_profile::get_by_id(repo, id)
}

pub fn get_tweets_by_author_id(repo: &mut Arc<Mutex<Repository>>, author_id: &str) -> Result<Vec<Tweet>> {
    println!("Get tweets written by {author_id}");
    repository::read_models::tweets::get_by_author_id(repo, author_id)
}

