use anyhow::Result;
use events::{
    Event, UserCreatedPayload, UserNumberTweetIncrementedPayload, UserTweetAddedPayload,
    UserTweetMessageEditedPayload,
};

use crate::event_sourcing::command::user::Command;

use super::Aggregate;

#[derive(Default)]
pub struct UserState {
    tweets_id: Vec<String>,
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
            Event::UserTweetAdded(payload) => Self::State {
                tweets_id: vec![state.tweets_id, vec![payload.tweet_id.clone()]].concat(),
            },
            Event::UserTweetMessageEdited(_) => state,
            Event::UserCreated(_) => state,
            Event::UserEdited(_) => todo!(),
            Event::UserNumberTweetIncremented(_) => state,
        }
    }

    fn run_command(state: Option<Self::State>, command: Command) -> Result<Vec<Self::Event>> {
        let events = match command {
            Command::CreateUser(data) => {
                if state.is_some() {
                    return Err(anyhow::anyhow!(
                        "User already exists. Id: {}",
                        &data.user_id
                    ));
                }
                // TODO: check nickname uniqueness
                let payload = UserCreatedPayload::new(data.user_id, data.nickname);
                vec![Event::UserCreated(payload)]
            }
            Command::AddTweet(data) => {
                let user_id = &data.author_id;
                let state = get_state_or_error(state, user_id)?;

                let payload =
                    UserTweetAddedPayload::new(user_id.to_owned(), data.message, data.tweet_id);
                let new_tweet_event = Event::UserTweetAdded(payload);

                let num_tweets: i32 = (state.tweets_id.len() + 1).try_into().unwrap();
                let payload = UserNumberTweetIncrementedPayload::new(user_id, num_tweets);
                let num_tweets_incremented = Event::UserNumberTweetIncremented(payload);
                vec![new_tweet_event, num_tweets_incremented]
            }
            Command::EditTweetMessage(data) => {
                let user_id = &data.author_id;
                let state = get_state_or_error(state, user_id)?;
                if !state.tweets_id.contains(&data.tweet_id) {
                    return Err(anyhow::anyhow!(
                        "Unable to edit unknown tweet: {}",
                        &data.tweet_id
                    ));
                }
                let event =
                    UserTweetMessageEditedPayload::new(user_id, &data.tweet_id, &data.message);
                vec![Event::UserTweetMessageEdited(event)]
            }
        };
        Ok(events)
    }
}

fn get_state_or_error(state: Option<UserState>, user_id: &str) -> Result<UserState> {
    state.ok_or_else(|| anyhow::anyhow!("Missing user for id {}", user_id))
}
