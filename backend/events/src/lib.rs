mod tweets;
mod user;

use serde::{Deserialize, Serialize};
pub use tweets::*;
pub use user::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Event {
    UserTweetAdded(tweets::UserTweetAddedPayload),
    UserNumberTweetIncremented(tweets::UserNumberTweetIncrementedPayload),
    UserTweetMessageEdited(tweets::UserTweetMessageEditedPayload),
    UserCreated(user::UserCreatedPayload),
    UserEdited(user::UserEditedPayload),
}

impl Event {
    pub fn event_type(&self) -> String {
        match self {
            Event::UserTweetAdded(_) => "user-tweet-added".to_owned(),
            Event::UserTweetMessageEdited(_) => "user-tweet-message-edited".to_owned(),
            Event::UserCreated(_) => "user-created".to_owned(),
            Event::UserEdited(_) => "user-edited".to_owned(),
            Event::UserNumberTweetIncremented(_) => "user-nummber-tweet-incremented".to_string(),
        }
    }

    pub fn aggregate_id(&self) -> String {
        match self {
            Event::UserTweetAdded(payload) => payload.id.to_owned(),
            Event::UserTweetMessageEdited(payload) => payload.id.to_owned(),
            Event::UserCreated(payload) => payload.id.to_owned(),
            Event::UserEdited(payload) => payload.id.to_owned(),
            Event::UserNumberTweetIncremented(payload) => payload.id.to_owned(),
        }
    }

    pub fn aggregate_type(&self) -> String {
        match self {
            Event::UserTweetAdded(_) => "users".to_owned(),
            Event::UserTweetMessageEdited(_) => "users".to_owned(),
            Event::UserCreated(_) => "users".to_owned(),
            Event::UserEdited(_) => "users".to_owned(),
            Event::UserNumberTweetIncremented(_) => "users".to_owned(),
        }
    }
}
