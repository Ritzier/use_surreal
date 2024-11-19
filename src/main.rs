use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("Surrealdb: {0}")]
    Surreal(#[from] surrealdb::Error),
    #[error("Dotenv: {0}")]
    Dotenv(#[from] dotenvy::Error),
    #[error("Var: {0}")]
    Var(#[from] std::env::VarError),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize)]
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

struct Config {
    ip_addr: String,
    port: String,
    username: String,
    password: String,
}

impl Config {
    fn load() -> Result<Self> {
        dotenv()?;

        let ip_addr = env::var("DB_IP")?;
        let port = env::var("DB_PORT")?;
        let username = env::var("DB_USER")?;
        let password = env::var("DB_PASS")?;

        Ok(Self {
            ip_addr,
            port,
            username,
            password,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    let db = Surreal::new::<Ws>(&format!("{}:{}", &config.ip_addr, &config.port)).await?;
    db.signin(Root {
        username: &config.username,
        password: &config.password,
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
