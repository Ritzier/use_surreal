use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InstagramUser {
    pub username: String,
    pub follower: u32,
    pub following: u32,
}
