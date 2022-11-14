use std::sync::{Arc, Mutex};

use anyhow::Result;
use repository::{self, read_models::tweets::Tweet, repo::Repository};
use uuid::Uuid;

pub fn add_tweet(repo: &mut Repository, author: &str, msg: &str) -> Result<String> {
    println!("{} is tweeting {:?}", author, msg);
    let id = Uuid::new_v4().to_string();
    repository::events::tweet::new(repo, id.to_owned(), author.to_owned(), msg.to_owned())?;
    println!("New tweet! {}", &id);
    Ok(id)
}

pub fn edit_tweet(repo: &mut Repository, id: &str, msg: &str) -> Result<()> {
    repository::events::tweet::edit(repo, id, msg)?;
    println!("Edited tweet!");
    Ok(())
}

pub fn get_by_id(repo: &mut Arc<Mutex<Repository>>, id: &str) -> Result<Tweet> {
    println!("Get tweet with id {id}");
    repository::read_models::tweets::get_by_id(repo, id)
}
