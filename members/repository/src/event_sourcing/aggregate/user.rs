use events::{Event, UserTweetAddedPayload, TweetMessageEditedPayload};

use crate::event_sourcing::command::user::Command;

use super::aggregate::Aggregate;

#[derive(Default)]
pub struct UserState {
    num_tweet: i32,
}

pub struct UserAggregate;

impl Aggregate for UserAggregate {
    type State = UserState;
    type Command = Command;
    type Event = Event;

    fn aggregate_type() -> &'static str {
        "users"
    }

    fn initial_state() -> Self::State {
        UserState::default()
    }

    fn handle_event(state: Self::State, event: &Self::Event) -> Self::State {
        match event {
            Event::TweetAdded(_) => Self::State {
                num_tweet: state.num_tweet + 1,
                ..state
            },
            Event::TweetMessageEdited(_) => state,
            Event::UserProfileAdded(_) => todo!(),
            Event::UserProfileEdited(_) => todo!(),
        }
    }

    fn run_command(_state: Option<Self::State>, command: Command) -> Vec<Self::Event> {
        match command {
            Command::AddTweet(data) => {
                let event = UserTweetAddedPayload::new(data.author_id, data.message, data.tweet_id);
                vec![Event::TweetAdded(event)]
            }
            Command::EditTweetMessage(data) => {
                let event = TweetMessageEditedPayload::new(&data.author_id, &data.tweet_id, &data.message);
                vec![Event::TweetMessageEdited(event)]
            }
        }
    }
}