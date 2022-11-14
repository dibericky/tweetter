use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use repository::read_models::tweets as ReadModel;

#[derive(SimpleObject)]
pub struct Tweet {
    pub id: String,
    pub author: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<ReadModel::Tweet> for Tweet {
    fn from(read_model: ReadModel::Tweet) -> Self {
        Self {
            id: read_model.id,
            author: read_model.author,
            message: read_model.message,
            created_at: read_model.created_at,
            updated_at: read_model.updated_at,
        }
    }
}
