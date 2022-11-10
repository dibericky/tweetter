use anyhow::Result;
use model::TweetData;
use repository::{repo::Repository, tweet};

pub fn add_tweet(repo: &mut Repository, author: &str, msg: &str) -> Result<()>{
    let data = TweetData::new(author.to_string(), msg.to_string());
    println!("Tweeting {:?}", data);
    
    tweet::new(repo, data)?;
    println!("New tweet!");
    Ok(())
}
