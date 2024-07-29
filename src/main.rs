use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize)]
struct User<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: u8,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Surreal::new::<Ws>("0.0.0.0:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;

    let person1 = User {
        first_name: "ritzier",
        last_name: "wheat",
        age: 255,
    };

    let a: Option<Record> = db.create(("user", "ritzier")).content(person1).await?;
    dbg!(a);

    Ok(())
}
