use anyhow::Result;

use crate::{event_sourcing, repo::Repository};

use super::{AddTweetPayload, EditTweetMessagePayload};

pub fn new(repo: &mut Repository, id: String, author: String, message: String) -> Result<()> {
    let cmd = event_sourcing::Command::AddTweet(AddTweetPayload::new(id, message, author));
    let events = event_sourcing::run_command(cmd);

    event_sourcing::store_events(repo, &events)?;

    Ok(())
}

pub fn edit(repo: &mut Repository, id: &str, msg: &str) -> Result<()> {
    let cmd = event_sourcing::Command::EditTweetMessage(EditTweetMessagePayload::new(id, msg));
    let events = event_sourcing::run_command(cmd);

    event_sourcing::store_events(repo, &events)?;

    Ok(())
}
