use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserProfileAddedPayload {
    pub id: String,
    pub nickname: String,
    num_tweets: i32,
    following: i32,
    follower: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserProfileAddedPayload {
    pub fn new(id: String, nickname: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            nickname,
            num_tweets: 0,
            follower: 0,
            following: 0,
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
pub struct UserProfileEditedPayload {
    pub id: String,
    pub nickname: String,
    updated_at: DateTime<Utc>,
}

impl UserProfileEditedPayload {
    pub fn new(id: &str, nickname: &str) -> Self {
        Self {
            id: id.to_owned(),
            nickname: nickname.to_owned(),
            updated_at: Utc::now(),
        }
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
