use std::error::Error;

use serde::{Deserialize, Serialize};

pub async fn get_zhubai(name: &str) -> Result<Vec<Post>, Box<dyn Error>> {
    let mut start = true;
    let mut res = Vec::new() as Vec<Post>;
    loop {
        let url = if start {
            format!("https://{}.zhubai.love/api/publications/{}/posts?publication_id_type=token&limit=10", name, name)
        } else {
            format!("https://{}.zhubai.love/api/publications/{}/posts?publication_id_type=token&created_at={}&limit=10", name, name, res.last().unwrap().created_at)
        };
        start = false;
        let resp = reqwest::get(url).await?.json::<Root>().await?;
        let resp_len = resp.data.len();
        // push data to res
        res.extend(resp.data);
        if resp_len < 10 {
            break;
        }
    }

    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    pub data: Vec<Post>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub author: Author,
    pub content: String,
    pub created_at: i64,
    pub id: String,
    pub is_paid_content: bool,
    pub paywall: Option<Paywall>,
    pub publication: Publication,
    pub title: String,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pagination {
    pub has_next: bool,
    pub has_prev: bool,
    pub next: String,
    pub prev: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Publication {
    pub created_at: i64,
    pub description: String,
    pub id: String,
    pub name: String,
    // pub theme: Option<_>,
    pub token: String,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paywall {
    pub end_at: i64,
    pub intro: String,
    pub start_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub avatar: String,
    pub description: String,
    pub id: String,
    pub name: String,
}


