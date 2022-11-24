use controller::{add_tweet, create_user};
use repository::repo::Repository;

fn main() {
    let mut repo = Repository::default();
    let user_id = create_user(&mut repo, "user1").expect("failed to create user");

    add_tweet(&mut repo, &user_id, "tweet msg 1").expect("failed to add tweet");
    add_tweet(&mut repo, &user_id, "tweet msg 2").expect("failed to add tweet");
}
