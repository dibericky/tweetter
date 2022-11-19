mod tweets;
mod user_profile;

use serde::{Deserialize, Serialize};
pub use tweets::*;
pub use user_profile::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Event {
    UserTweetAdded(tweets::UserTweetAddedPayload),
    UserTweetMessageEdited(tweets::UserTweetMessageEditedPayload),
    UserProfileAdded(user_profile::UserProfileAddedPayload),
    UserProfileEdited(user_profile::UserProfileEditedPayload),
}

impl Event {
    pub fn event_type(&self) -> String {
        match self {
            Event::UserTweetAdded(_) => "user-tweet-added".to_owned(),
            Event::UserTweetMessageEdited(_) => "user-tweet-message-edited".to_owned(),
            Event::UserProfileAdded(_) => todo!(),
            Event::UserProfileEdited(_) => todo!(),
        }
    }

    pub fn aggregate_id(&self) -> String {
        match self {
            Event::UserTweetAdded(payload) => payload.id.to_owned(),
            Event::UserTweetMessageEdited(payload) => payload.id.to_owned(),
            Event::UserProfileAdded(_) => todo!(),
            Event::UserProfileEdited(_) => todo!(),
        }
    }

    pub fn aggregate_type(&self) -> String {
        match self {
            Event::UserTweetAdded(_) => "users".to_owned(),
            Event::UserTweetMessageEdited(_) => "users".to_owned(),
            Event::UserProfileAdded(_) => "users".to_owned(),
            Event::UserProfileEdited(_) => "users".to_owned(),
        }
    }
}
