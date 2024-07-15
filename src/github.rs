use std::fmt::Display;

use reqwest::{
    header::{HeaderMap, ACCEPT, USER_AGENT},
    IntoUrl,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("Header Error: {0:?}")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Request Error: {0:?}")]
    RequestError(#[from] reqwest::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitUser {
    pub name: String,
    pub public_repos: u32,
}

impl GitUser {
    pub async fn get_latest(name: impl IntoUrl + Display) -> Result<GitUser, GitError> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "User".parse()?);
        headers.insert(ACCEPT, "application/vnd.github+json".parse()?);

        let client = reqwest::Client::new();
        let result: GitUser = client
            .get(&format!("https://api.github.com/users/{}", name))
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }
}
