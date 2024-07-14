use std::env;

use super::error::DatabaseError;

use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

pub struct Database(pub Surreal<Client>);

impl Database {
    /// Required `DB_USER`, `DB_PASS`, `DB_PORT` from environment
    /// Sigin as ROOT, and return
    pub async fn init() -> Result<Database, DatabaseError> {
        dotenv()?;

        let db_user = env::var("DB_USER")?;
        let db_pass = env::var("DB_PASS")?;
        let db_port = env::var("DB_PORT")?;
        let db = Surreal::new::<Ws>(&format!("127.0.0.1:{}", db_port)).await?;

        db.signin(Root {
            username: &db_user,
            password: &db_pass,
        })
        .await?;

        db.use_ns("test").use_db("test").await?;

        Ok(Database(db))
    }

    /// Switch to specific
    pub async fn use_ns(&self, namespace: impl Into<String>) -> Result<(), DatabaseError> {
        self.0.use_ns(namespace).await?;
        Ok(())
    }

    /// Swith to specific namespace
    pub async fn use_db(&self, database: impl Into<String>) -> Result<(), DatabaseError> {
        self.0.use_db(database).await?;
        Ok(())
    }

    pub async fn insert<T: Serialize>(
        &self,
        item: &T,
        table: (&str, &str),
    ) -> Result<Record, DatabaseError> {
        let result: Option<Record> = self.0.insert(table).content(item).await?;
        Ok(result.expect("Error"))
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        table: (&str, &str),
    ) -> Result<Option<T>, DatabaseError> {
        let result: Option<T> = self.0.select(table).await?;
        Ok(result)
    }

    pub async fn get_all<T: for<'de> Deserialize<'de>>(
        &self,
        table: &str,
    ) -> Result<Vec<T>, DatabaseError> {
        let result: Vec<T> = self.0.select(table).await?;
        Ok(result)
    }
}
