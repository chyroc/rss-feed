use std::error::Error;

use reqwest;
use serde::{Deserialize, Serialize};

pub async fn get_feeds() -> Result<Vec<Feed>, Box<dyn Error>> {
    let items = reqwest::get("https://www.v2ex.com/api/topics/latest.json")
        .await?.json::<Vec<Feed>>().await?;
    Ok(items)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    pub node: Node,
    pub member: Member,
    pub last_touched: i32,
    pub title: String,
    pub url: String,
    pub created: i64,
    pub content: String,
    pub content_rendered: String,
    pub last_modified: i32,
    pub replies: i32,
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub avatar_large: String,
    pub name: String,
    pub avatar_normal: String,
    pub title: String,
    pub url: String,
    pub topics: i32,
    pub header: String,
    pub title_alternative: String,
    pub avatar_mini: String,
    pub stars: i32,
    pub id: i32,
    pub parent_node_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub username: String,
    pub avatar_normal: String,
    pub bio: Option<String>,
    pub url: String,
    pub created: i32,
    pub avatar_large: String,
    pub avatar_mini: String,
    pub location: Option<String>,
    pub id: i32,
}


