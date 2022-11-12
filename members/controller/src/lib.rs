use anyhow::Result;
use model::Tweet;
use repository::{repo::Repository, self};

pub fn add_tweet(repo: &mut Repository, author: &str, msg: &str) -> Result<String>{
    let id = uuid::Uuid::new_v4();
    let id = id.to_string();
    let data = Tweet::new(id.clone(), author.to_string(), msg.to_string());
    println!("Tweeting {:?}", data);
    
    repository::events::tweet::new(repo, data)?;
    println!("New tweet!");
    Ok(id)
}

pub fn edit_tweet(repo: &mut Repository, id: &str, msg: &str) -> Result<()>{
    repository::events::tweet::edit(repo, id, msg)?;
    println!("Edited tweet!");
    Ok(())
}
