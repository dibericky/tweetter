use model::{TweetData, Tweet};

use crate::repo::{Repository, Table};

pub fn new(repo: &Repository, data: TweetData) -> Result<Tweet, String> {
    let id = repo.add(Table::TWEETS, &data);
    Ok((id, data).into())
}

