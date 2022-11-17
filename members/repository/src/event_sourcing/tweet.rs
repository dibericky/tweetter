use anyhow::Result;

use crate::event_sourcing::command::user as UserCommand;
use crate::{
    event_sourcing::{
        self,
        aggregate::{user::UserAggregate, Aggregate},
    },
    repo::Repository,
};

pub fn new(
    repo: &mut Repository,
    tweet_id: String,
    author_id: String,
    message: String,
) -> Result<()> {
    let aggregate_id = author_id.clone();
    let cmd = UserCommand::Command::AddTweet(UserCommand::AddTweetPayload::new(
        author_id, message, tweet_id,
    ));
    let state = UserAggregate::load(repo, &aggregate_id)?;
    let events = UserAggregate::run_command(state, cmd);

    event_sourcing::store_events(repo, &events)?;

    Ok(())
}

pub fn edit(repo: &mut Repository, tweet_id: &str, author_id: &str, msg: &str) -> Result<()> {
    let aggregate_id = author_id;
    let cmd = UserCommand::Command::EditTweetMessage(UserCommand::EditTweetMessagePayload::new(
        author_id, tweet_id, msg,
    ));
    let state = UserAggregate::load(repo, aggregate_id)?;
    let events = UserAggregate::run_command(state, cmd);

    event_sourcing::store_events(repo, &events)?;

    Ok(())
}
