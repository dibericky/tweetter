use anyhow::Result;
use events::Event;
use repository::repo::Repository;

pub fn update(_repo: &Repository, event: &Event) -> Result<()> {
    match event {
        Event::TweetAdded(payload) => {

        },
        Event::TweetMessageEdited(payload) => {
            
        },
    };
    Ok(())
}
