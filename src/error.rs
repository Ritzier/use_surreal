use std::env;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("`.env` error: {0:?}")]
    DotEnvError(#[from] dotenvy::Error),

    #[error("Error get environment: {0:?}")]
    VarError(#[from] env::VarError),

    #[error("SurrealDB error: {0:?}")]
    SurrealError(#[from] surrealdb::Error),
}
