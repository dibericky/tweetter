use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Event {
    TweetAdded(TweetAddedPayload),
    TweetMessageEdited(TweetMessageEditedPayload),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TweetAddedPayload {
    pub id: String,
    pub message: String,
    pub author: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TweetAddedPayload {
    pub fn new(id: String, message: String, author: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            message,
            author,
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
    pub message: String,
    updated_at: DateTime<Utc>,
}

impl TweetMessageEditedPayload {
    pub fn new(id: &str, message: &str) -> Self {
        Self {
            id: id.to_owned(),
            message: message.to_owned(),
            updated_at: Utc::now(),
        }
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

impl Event {
    pub fn event_type(&self) -> String {
        match self {
            Event::TweetAdded(_) => "tweet-added".to_owned(),
            Event::TweetMessageEdited(_) => "tweet-message-edited".to_owned(),
        }
    }

    pub fn aggregate_id(&self) -> String {
        match self {
            Event::TweetAdded(payload) => payload.id.to_owned(),
            Event::TweetMessageEdited(payload) => payload.id.to_owned(),
        }
    }
}
