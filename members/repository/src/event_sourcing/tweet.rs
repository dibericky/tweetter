use anyhow::Result;

use crate::{event_sourcing, repo::Repository};

use super::{AddTweetPayload, EditTweetMessagePayload};

pub fn new(repo: &mut Repository, id: String, author_id: String, message: String) -> Result<()> {
    let cmd = event_sourcing::Command::AddTweet(AddTweetPayload::new(id, message, author_id));
    let events = event_sourcing::run_command(cmd);

    event_sourcing::store_events(repo, &events)?;

    Ok(())
}

pub fn edit(repo: &mut Repository, id: &str, author_id: &str, msg: &str) -> Result<()> {
    let cmd =
        event_sourcing::Command::EditTweetMessage(EditTweetMessagePayload::new(id, author_id, msg));
    let events = event_sourcing::run_command(cmd);

    event_sourcing::store_events(repo, &events)?;

    Ok(())
}
