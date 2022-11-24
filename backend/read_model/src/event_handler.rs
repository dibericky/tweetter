use std::sync::{Arc, Mutex};

use anyhow::Result;
use events::Event;
use repository::repo::Repository;

pub fn event_handler(repo: &mut Arc<Mutex<Repository>>, event: &Event) -> Result<()> {
    crate::tweets::update(repo, event)?;
    crate::users::update(repo, event)?;
    Ok(())
}
