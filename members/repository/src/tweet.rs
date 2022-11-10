use anyhow::Result;
use model::{TweetData};

use crate::{repo::{Repository}, event_sourcing};

pub fn new(repo: &mut Repository, data: TweetData) -> Result<()> {
    let cmd = event_sourcing::Command::AddTweet(data);
    let events = event_sourcing::run_command(cmd);
    
    event_sourcing::store_events(repo, &events);

    Ok(())
}

