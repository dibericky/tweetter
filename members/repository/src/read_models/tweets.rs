use std::sync::{Arc, Mutex};

use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::ExpressionMethods;
use diesel::{AsChangeset, Insertable, QueryDsl, Queryable, RunQueryDsl};
use events::{UserTweetAddedPayload, UserTweetMessageEditedPayload};

use crate::{
    repo::Repository,
    schema::{
        self,
        tweets::{self as TweetsSchema},
    },
};

#[derive(Insertable, Queryable)]
#[diesel(table_name = TweetsSchema)]
pub struct Tweet {
    pub id: String,
    pub author_id: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, AsChangeset)]
#[diesel(table_name = TweetsSchema)]
pub struct UpdateTweet {
    pub message: String,
    pub updated_at: DateTime<Utc>,
}

pub fn insert(repo: &mut Arc<Mutex<Repository>>, doc: Tweet) -> Result<()> {
    let mut repo_locked = repo.lock().unwrap();
    let inserted = diesel::insert_into(schema::tweets::table)
        .values(&doc)
        .execute(repo_locked.get_connection())
        .map_err(|_| anyhow::anyhow!("Failed to insert tweet"))?;

    println!("Inserted {:?}", inserted);
    Ok(())
}

pub fn get_by_id(repo: &mut Arc<Mutex<Repository>>, id: &str) -> Result<Tweet> {
    let mut repo_locked = repo.lock().unwrap();
    schema::tweets::dsl::tweets
        .find(id)
        .first(repo_locked.get_connection())
        .map_err(|_| anyhow::anyhow!("Failed to get tweet"))
}

pub fn update(repo: &mut Arc<Mutex<Repository>>, id: &str, values: UpdateTweet) -> Result<()> {
    let mut repo_locked = repo.lock().unwrap();
    let updated = diesel::update(schema::tweets::dsl::tweets.filter(TweetsSchema::dsl::id.eq(id)))
        .set(values)
        .execute(repo_locked.get_connection())
        .map_err(|_| anyhow::anyhow!("Failed to update tweet"))?;
    println!("Updated {:?}", updated);
    Ok(())
}

impl From<&UserTweetAddedPayload> for Tweet {
    fn from(tweet: &UserTweetAddedPayload) -> Self {
        Self {
            id: tweet.tweet_id.to_owned(),
            author_id: tweet.id.to_owned(),
            message: tweet.message.to_owned(),
            created_at: tweet.occurred_on().to_owned(),
            updated_at: tweet.occurred_on().to_owned(),
        }
    }
}

impl From<&UserTweetMessageEditedPayload> for UpdateTweet {
    fn from(data: &UserTweetMessageEditedPayload) -> Self {
        Self {
            message: data.message.to_owned(),
            updated_at: data.occurred_on().to_owned(),
        }
    }
}
