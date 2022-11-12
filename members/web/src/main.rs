use anyhow::Result;
use repository::repo::Repository;

fn main() -> Result<()> {
    let mut repo = Repository::default();
    controller::add_tweet(&mut repo, "@dibericky",  "Hi there! Check file members/web/src/main.rs")?;

    // controller::edit_tweet(
    //     &mut repo,
    //     "b89e6b1b-21fd-49bb-8a0c-3cd697b81142",
    //     "This is another changed with mb!",
    // )?;

    Ok(())
}
