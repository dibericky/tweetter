use model::Tweet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag="type")]
pub enum Event {
    TweetAdded(Tweet)
}

impl Event {
    pub fn event_type(&self) -> String {
        match self {
            Event::TweetAdded(_) => "tweet-added".to_owned(),
        }
    }

    pub fn aggregate_id(&self) -> String {
        match self {
            Event::TweetAdded(payload) =>  payload.id.to_owned(),
        }
    }
}
