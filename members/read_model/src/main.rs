use std::sync::{Arc, Mutex};

use anyhow::Result;
use events::Event;
use repository::repo::Repository;

fn main() -> Result<()> {
    let repo = Arc::new(Mutex::new(Repository::default()));
    message_broker::consume("tweets", |event: &Event| {
        let mut repo = repo.clone();
        read_model::tweets::update(&mut repo, event)?;

        Ok(())
    })?;
    Ok(())
}
