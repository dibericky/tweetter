mod tweets;
mod user_profile;

use serde::{Deserialize, Serialize};
pub use tweets::*;
pub use user_profile::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Event {
    TweetAdded(tweets::UserTweetAddedPayload),
    TweetMessageEdited(tweets::TweetMessageEditedPayload),
    UserProfileAdded(user_profile::UserProfileAddedPayload),
    UserProfileEdited(user_profile::UserProfileEditedPayload),
}

impl Event {
    pub fn event_type(&self) -> String {
        match self {
            Event::TweetAdded(_) => "tweet-added".to_owned(),
            Event::TweetMessageEdited(_) => "tweet-message-edited".to_owned(),
            Event::UserProfileAdded(_) => todo!(),
            Event::UserProfileEdited(_) => todo!(),
        }
    }

    pub fn aggregate_id(&self) -> String {
        match self {
            Event::TweetAdded(payload) => payload.id.to_owned(),
            Event::TweetMessageEdited(payload) => payload.id.to_owned(),
            Event::UserProfileAdded(_) => todo!(),
            Event::UserProfileEdited(_) => todo!(),
        }
    }

    pub fn aggregate_type(&self) -> String {
        match self {
            Event::TweetAdded(_) => "users".to_owned(),
            Event::TweetMessageEdited(_) => "users".to_owned(),
            Event::UserProfileAdded(_) => "users".to_owned(),
            Event::UserProfileEdited(_) => "users".to_owned(),
        }
    }
}
