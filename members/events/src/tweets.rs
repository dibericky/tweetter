use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TweetAddedPayload {
    pub id: String,
    pub message: String,
    pub author_id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TweetAddedPayload {
    pub fn new(id: String, message: String, author_id: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            message,
            author_id,
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
pub struct TweetMessageEditedPayload {
    pub id: String,
    pub author_id: String,
    pub message: String,
    updated_at: DateTime<Utc>,
}

impl TweetMessageEditedPayload {
    pub fn new(id: &str, author_id: &str, message: &str) -> Self {
        Self {
            id: id.to_owned(),
            author_id: author_id.to_string(),
            message: message.to_owned(),
            updated_at: Utc::now(),
        }
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
