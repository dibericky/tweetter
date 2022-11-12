use std::sync::{Arc, Mutex};

use anyhow::Result;
use events::Event;
use read_model::tweets::update;
use repository::repo::Repository;

fn main() -> Result<()> {
    let repo = Arc::new(Mutex::new(Repository::default()));
    message_broker::consume("tweets", |event: &Event| {
        let mut repo = repo.clone();
        update(&mut repo, event)
    })?;
    Ok(())
}
