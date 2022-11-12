use model::{Tweet, TweetID, TweetMessage};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag="type")]
pub enum Event {
    TweetAdded(TweetAddedPayload),
    TweetMessageEdited(TweetMessageEditedPayload)
}

pub type TweetAddedPayload = Tweet;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TweetMessageEditedPayload {
    id: TweetID,
    message: TweetMessage
}

impl From<(TweetID, TweetMessage)> for TweetMessageEditedPayload {
    fn from((id, message): (TweetID, TweetMessage)) -> Self {
        Self { id, message }
    }
}

impl Event {
    pub fn event_type(&self) -> String {
        match self {
            Event::TweetAdded(_) => "tweet-added".to_owned(),
            Event::TweetMessageEdited(_) => "tweet-message-edited".to_owned(),
        }
    }

    pub fn aggregate_id(&self) -> String {
        match self {
            Event::TweetAdded(payload) =>  payload.id.to_owned(),
            Event::TweetMessageEdited(payload) => payload.id.to_owned()
        }
    }
}
