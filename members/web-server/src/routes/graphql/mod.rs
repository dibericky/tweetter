use std::sync::{Arc, Mutex};

use async_graphql::{EmptySubscription, Object, Result, Schema};
use repository::{repo::Repository};

use self::{tweet::Tweet as GraphqlTweet, user::User};

mod tweet;
mod user;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn tweet(&self, tweet_id: String) -> Result<GraphqlTweet> {
        let mut repo = Arc::new(Mutex::new(Repository::default()));
        let tweet = controller::get_tweet(&mut repo, &tweet_id)?;
        Ok(tweet.into())
    }

    pub async fn user(&self, user_id: String) -> Result<User> {
        let mut repo = Arc::new(Mutex::new(Repository::default()));
        let user = controller::get_user(&mut repo, &user_id)?;
        Ok(user.into())
    }

    pub async fn user_tweets(&self, author_id: String) -> Result<Vec<GraphqlTweet>> {
        let mut repo = Arc::new(Mutex::new(Repository::default()));
        let tweets = controller::get_tweets_by_author_id(&mut repo, &author_id)?;
        let tweets = tweets
            .into_iter()
            .map(GraphqlTweet::from)
            .collect::<Vec<_>>();
        Ok(tweets)
    }
}

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    pub async fn add_tweet(&self, user_id: String, message: String) -> Result<String> {
        let mut repo = Repository::default();
        let id = controller::add_tweet(&mut repo, &user_id, &message)?;
        Ok(id)
    }

    pub async fn edit_tweet(
        &self,
        tweet_id: String,
        user_id: String,
        message: String,
    ) -> Result<bool> {
        let mut repo = Repository::default();
        controller::edit_tweet(&mut repo, &tweet_id, &user_id, &message)?;
        Ok(true)
    }

    pub async fn create_user(&self, nickname: String) -> Result<String> {
        let mut repo = Repository::default();
        let id = controller::create_user(&mut repo, &nickname)?;
        Ok(id)
    }
}

pub type TweetterSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
