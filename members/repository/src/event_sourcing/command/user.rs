pub struct AddTweetPayload {
    pub author_id: String,
    pub tweet_id: String,
    pub message: String,
}

impl AddTweetPayload {
    pub fn new(author_id: String, message: String, tweet_id: String) -> Self {
        Self {
            tweet_id,
            author_id,
            message,
        }
    }
}

pub struct EditTweetMessagePayload {
    pub author_id: String,
    pub tweet_id: String,
    pub message: String,
}

impl EditTweetMessagePayload {
    pub fn new(author_id: &str, tweet_id: &str, message: &str) -> Self {
        Self {
            message: message.to_owned(),
            author_id: author_id.to_owned(),
            tweet_id: tweet_id.to_owned(),
        }
    }
}

pub enum Command {
    AddTweet(AddTweetPayload),
    EditTweetMessage(EditTweetMessagePayload),
}
