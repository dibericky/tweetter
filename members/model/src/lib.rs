use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
    id: String,
    data: TweetData
}

impl Tweet {
    pub fn new(id: String, data: TweetData) -> Self {
        Self {id, data}
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TweetData {
    author: String,
    message: String
}

impl TweetData {
    pub fn new(author: String, message: String) -> Self {
        Self {author, message}
    }
}

impl From<(String, TweetData)> for Tweet {
    fn from((id, data): (String, TweetData)) -> Self {
        Self {
            id,
            data
        }
    }
}
