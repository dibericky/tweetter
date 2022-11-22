use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserTweetAddedPayload {
    pub id: String,
    pub message: String,
    pub tweet_id: String,
    occurred_on: DateTime<Utc>,
}

impl UserTweetAddedPayload {
    pub fn new(id: String, message: String, tweet_id: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            message,
            tweet_id,
            occurred_on: now,
        }
    }

    pub fn occurred_on(&self) -> &DateTime<Utc> {
        &self.occurred_on
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserTweetMessageEditedPayload {
    pub id: String,
    pub tweet_id: String,
    pub message: String,
    occurred_on: DateTime<Utc>,
}

impl UserTweetMessageEditedPayload {
    pub fn new(id: &str, tweet_id: &str, message: &str) -> Self {
        Self {
            tweet_id: tweet_id.to_owned(),
            id: id.to_string(),
            message: message.to_owned(),
            occurred_on: Utc::now(),
        }
    }

    pub fn occurred_on(&self) -> &DateTime<Utc> {
        &self.occurred_on
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserNumberTweetIncrementedPayload {
    pub id: String,
    pub num_tweet: i32,
    occurred_on: DateTime<Utc>,
}

impl UserNumberTweetIncrementedPayload {
    pub fn new(id: &str, num_tweet: i32) -> Self {
        Self {
            id: id.to_string(),
            num_tweet,
            occurred_on: Utc::now(),
        }
    }

    pub fn occurred_on(&self) -> &DateTime<Utc> {
        &self.occurred_on
    }
}
