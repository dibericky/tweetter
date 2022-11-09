use repository::repo::Repository;


fn main() -> Result<(), String>{
    let repo = Repository::new();
    let msg = "Hi there!";
    controller::add_tweet(&repo, "@dibericky", msg)?;

    Ok(())
}
