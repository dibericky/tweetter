use model::TweetData;
use repository::{repo::Repository, tweet};

pub fn add_tweet(repo: &Repository, author: &str, msg: &str) -> Result<(), String>{
    let data = TweetData::new(author.to_string(), msg.to_string());
    println!("Tweeting {:?}", data);
    let new_tweet = tweet::new(repo, data)?;
    println!("New tweet! {:?}", new_tweet);
    Ok(())
}
