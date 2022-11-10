use anyhow::Result;
use repository::repo::Repository;


fn main() -> Result<()>{
    let mut repo = Repository::new();
    let msg = "Hi there!";
    controller::add_tweet(&mut repo, "@dibericky", msg)?;

    Ok(())
}
