use anyhow::Result;

use crate::event_sourcing;
use crate::{event_sourcing::command::user as UserCommand, repo::Repository};

use super::aggregate::user::UserAggregate;
use super::aggregate::Aggregate;

pub fn new(repo: &mut Repository, user_id: &str, nickname: &str) -> Result<()> {
    let aggregate_id = user_id;
    let cmd = UserCommand::Command::CreateUser(UserCommand::CreateUserPayload::new(
        user_id.to_string(),
        nickname.to_string(),
    ));
    let state = UserAggregate::load(repo, aggregate_id)?;
    let events = UserAggregate::run_command(state, cmd)?;

    event_sourcing::store_events(repo, &events)?;

    Ok(())
}
