use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
    pub id: TweetID,
    author: String,
    message: TweetMessage
}

impl Tweet {
    pub fn new(id: String, author: String, message: String) -> Self {
        Self {id, author, message}
    }
}

pub type TweetID = String;
pub type TweetMessage = String; 
