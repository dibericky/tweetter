use anyhow::Result;
use model::Tweet;

use crate::{repo::{Repository}, event_sourcing};

pub fn new(repo: &mut Repository, data: Tweet) -> Result<()> {
    let cmd = event_sourcing::Command::AddTweet(data);
    let events = event_sourcing::run_command(cmd);
    
    event_sourcing::store_events(repo, &events);

    Ok(())
}

pub fn edit(repo: &mut Repository, id: &str, msg: &str) -> Result<()> {
    let cmd = event_sourcing::Command::EditTweet(id.to_string(), msg.to_string());
    let events = event_sourcing::run_command(cmd);
    
    event_sourcing::store_events(repo, &events);

    Ok(())
}
