use anyhow::Result;
use events::Event;
use read_model::tweets::update;
use repository::repo::Repository;

fn main() -> Result<()> {
    let repo = Repository::new();
    message_broker::consume("tweets", |event: &Event| {
        update(&repo, event)
    })?;
    Ok(())
}