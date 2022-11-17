use std::sync::{Arc, Mutex};

use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl};

use crate::{
    repo::Repository,
    schema::{self, user_profile as UserProfileSchema},
};

#[derive(Insertable, Queryable)]
#[diesel(table_name = UserProfileSchema)]
pub struct UserProfile {
    pub id: String,
    pub nickname: String,
    pub num_tweets: i32,
    pub following: i32,
    pub follower: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = UserProfileSchema)]
struct UpdateSetUserProfile {
    pub id: String,
    pub nickname: Option<String>,
    pub num_tweets: Option<i32>,
    pub following: Option<i32>,
    pub follower: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

pub struct UpdateUserProfile {
    pub nickname: Option<String>,
    pub num_tweets: Option<i32>,
    pub following: Option<i32>,
    pub follower: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

impl From<(&str, UpdateUserProfile)> for UpdateSetUserProfile {
    fn from((id, data): (&str, UpdateUserProfile)) -> Self {
        Self {
            id: id.to_owned(),
            nickname: data.nickname,
            updated_at: data.updated_at,
            num_tweets: data.num_tweets,
            following: data.following,
            follower: data.follower,
        }
    }
}

pub fn insert(repo: &mut Arc<Mutex<Repository>>, doc: UserProfile) -> Result<()> {
    let mut repo_locked = repo.lock().unwrap();
    diesel::insert_into(schema::user_profile::table)
        .values(&doc)
        .execute(repo_locked.get_connection())
        .map_err(|_| anyhow::anyhow!("Failed to insert user profile"))?;
    Ok(())
}

pub fn get_by_id(repo: &mut Arc<Mutex<Repository>>, id: &str) -> Result<UserProfile> {
    let mut repo_locked = repo.lock().unwrap();
    schema::user_profile::dsl::user_profile
        .find(id)
        .first(repo_locked.get_connection())
        .map_err(|_| anyhow::anyhow!("Failed to get user profile"))
}

pub fn update(repo: &mut Arc<Mutex<Repository>>, id: &str, doc: UpdateUserProfile) -> Result<()> {
    let values: UpdateSetUserProfile = (id, doc).into();
    let mut repo_locked = repo.lock().unwrap();
    diesel::update(schema::user_profile::table)
        .set(&values)
        .execute(repo_locked.get_connection())
        .map_err(|_| anyhow::anyhow!("Failed to update user profile"))?;
    Ok(())
}
