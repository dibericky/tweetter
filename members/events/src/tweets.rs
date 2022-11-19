use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserTweetAddedPayload {
    pub id: String,
    pub message: String,
    pub tweet_id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserTweetAddedPayload {
    pub fn new(id: String, message: String, tweet_id: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            message,
            tweet_id,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserTweetMessageEditedPayload {
    pub id: String,
    pub tweet_id: String,
    pub message: String,
    updated_at: DateTime<Utc>,
}

impl UserTweetMessageEditedPayload {
    pub fn new(id: &str, tweet_id: &str, message: &str) -> Self {
        Self {
            tweet_id: tweet_id.to_owned(),
            id: id.to_string(),
            message: message.to_owned(),
            updated_at: Utc::now(),
        }
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
