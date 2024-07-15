mod database;
mod error;
mod github;
mod instagram;

use database::Database;
use github::GitUser;
use instagram::InstagramUser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = Database::init().await?;
    database.use_ns("test").await?;
    database.use_db("test").await?;

    let git = GitUser::get_latest("ritzier").await?;
    let result = database.insert(&git, ("git", "ritzier")).await?;
    dbg!(result);

    Ok(())
}
