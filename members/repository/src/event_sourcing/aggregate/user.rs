use events::Event;

use super::aggregate::Aggregate;

#[derive(Default)]
pub struct UserState {
    num_tweet: i32,
}

pub struct UserAggregate;

impl Aggregate for UserAggregate {
    type State = UserState;

    fn aggregate_type() -> &'static str {
        "users"
    }

    fn initial_state() -> Self::State {
        UserState::default()
    }

    fn handle_event(state: Self::State, event: &Event) -> Self::State {
        match event {
            Event::TweetAdded(_) => Self::State {
                num_tweet: state.num_tweet + 1,
                ..state
            },
            Event::TweetMessageEdited(_) => todo!(),
            Event::UserProfileAdded(_) => todo!(),
            Event::UserProfileEdited(_) => todo!(),
        }
    }
}
