use std::sync::{Arc, Mutex};

use async_graphql::{EmptySubscription, Object, Result, Schema};
use repository::repo::Repository;

use self::tweet::Tweet;

mod tweet;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn tweet(&self, tweet_id: String) -> Result<Tweet> {
        let mut repo = Arc::new(Mutex::new(Repository::default()));
        let tweet = controller::get_by_id(&mut repo, &tweet_id)?;
        Ok(tweet.into())
    }
}

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    pub async fn add_tweet(&self, user: String, message: String) -> Result<String> {
        let mut repo = Repository::default();
        let id = controller::add_tweet(&mut repo, &user, &message)?;
        Ok(id)
    }

    pub async fn edit_tweet(&self, tweet_id: String, message: String) -> Result<bool> {
        let mut repo = Repository::default();
        controller::edit_tweet(&mut repo, &tweet_id, &message)?;
        Ok(true)
    }
}

pub type TweetterSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
