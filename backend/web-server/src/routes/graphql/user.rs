use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use repository::read_models::user_profile as ReadModel;

#[derive(SimpleObject)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub num_tweets: i32,
    pub following: i32,
    pub follower: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ReadModel::UserProfile> for User {
    fn from(read_model: ReadModel::UserProfile) -> Self {
        Self {
            id: read_model.id,
            nickname: read_model.nickname,
            num_tweets: read_model.num_tweets,
            following: read_model.following,
            follower: read_model.follower,
            created_at: read_model.created_at,
            updated_at: read_model.updated_at,
        }
    }
}
