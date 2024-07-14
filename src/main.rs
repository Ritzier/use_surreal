mod database;
mod error;
mod instagram;

use database::Database;
use instagram::InstagramUser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = Database::init().await?;
    database.use_ns("test").await?;
    database.use_db("test").await?;

    let user = InstagramUser {
        username: String::from("Ritzier"),
        follower: 10,
        following: 1,
    };

    let result = database.insert(&user, ("person", &user.username)).await?;
    dbg!(result);
    let result = database.insert(&user, ("person", &user.username)).await?;
    dbg!(result);

    let a: Vec<InstagramUser> = database.get_all("person").await?;
    dbg!(a);

    let a: Option<InstagramUser> = database.get(("person", "Ritzier")).await?;
    dbg!(a);

    Ok(())
}
