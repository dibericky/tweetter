use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCreatedPayload {
    pub id: String,
    pub nickname: String,
    num_tweets: i32,
    following: i32,
    follower: i32,
    occurred_on: DateTime<Utc>,
}

impl UserCreatedPayload {
    pub fn new(id: String, nickname: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            nickname,
            num_tweets: 0,
            follower: 0,
            following: 0,
            occurred_on: now,
        }
    }

    pub fn occurred_on(&self) -> &DateTime<Utc> {
        &self.occurred_on
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserEditedPayload {
    pub id: String,
    pub nickname: String,
    occurred_on: DateTime<Utc>,
}

impl UserEditedPayload {
    pub fn new(id: &str, nickname: &str) -> Self {
        Self {
            id: id.to_owned(),
            nickname: nickname.to_owned(),
            occurred_on: Utc::now(),
        }
    }

    pub fn occurred_on(&self) -> &DateTime<Utc> {
        &self.occurred_on
    }
}
