use std::sync::{Arc, Mutex};

use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, RunQueryDsl};
use events::{TweetAddedPayload, TweetMessageEditedPayload};

use crate::schema::tweets::dsl as Fields;
use crate::{
    repo::Repository,
    schema::{self, tweets as TweetsSchema},
};

#[derive(Insertable)]
#[diesel(table_name = TweetsSchema)]
pub struct InsertTweet {
    pub id: String,
    pub author: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = TweetsSchema)]
struct UpdateSetTweet {
    pub id: String,
    pub message: String,
    pub updated_at: DateTime<Utc>,
}

pub struct UpdateTweet {
    pub message: String,
    pub updated_at: DateTime<Utc>,
}

impl From<(&str, UpdateTweet)> for UpdateSetTweet {
    fn from((id, data): (&str, UpdateTweet)) -> Self {
        Self {
            id: id.to_owned(),
            message: data.message,
            updated_at: data.updated_at,
        }
    }
}

pub fn insert(repo: &mut Arc<Mutex<Repository>>, doc: InsertTweet) -> Result<()> {
    let mut repo_locked = repo.lock().unwrap();
    diesel::insert_into(schema::tweets::table)
        .values(&doc)
        .execute(repo_locked.get_connection())
        .map_err(|_| anyhow::anyhow!("Failed to store event"))?;
    Ok(())
}

pub fn update(repo: &mut Arc<Mutex<Repository>>, id: &str, doc: UpdateTweet) -> Result<()> {
    let values: UpdateSetTweet = (id, doc).into();
    let mut repo_locked = repo.lock().unwrap();
    diesel::update(schema::tweets::table)
        .set(&values)
        .execute(repo_locked.get_connection())
        .map_err(|_| anyhow::anyhow!("Failed to store event"))?;
    Ok(())
}

impl From<&TweetAddedPayload> for InsertTweet {
    fn from(tweet: &TweetAddedPayload) -> Self {
        Self {
            id: tweet.id.to_owned(),
            author: tweet.author.to_owned(),
            message: tweet.message.to_owned(),
            created_at: tweet.created_at().to_owned(),
            updated_at: tweet.updated_at().to_owned(),
        }
    }
}

impl From<&TweetMessageEditedPayload> for UpdateTweet {
    fn from(data: &TweetMessageEditedPayload) -> Self {
        Self {
            message: data.message.to_owned(),
            updated_at: data.updated_at().to_owned(),
        }
    }
}
