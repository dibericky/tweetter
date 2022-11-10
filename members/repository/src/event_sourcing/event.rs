use model::Tweet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag="type")]
pub enum Event {
    TweetAdded(Tweet)
}

impl Event {
    pub fn event_type(&self) -> &str {
        match self {
            Event::TweetAdded(_) => "tweet-added",
        }
    }
}