use std::error::Error;

use serde::{Deserialize, Serialize};

pub async fn get_info(name: &str) -> Result<SiteInfo, Box<dyn Error>> {
    let url = format!("https://{}.zhubai.love/api/publications/{}?id_type=token", name, name);
    let resp = reqwest::get(url).await?.json::<SiteInfo>().await?;
    Ok(resp)
}

pub async fn get_feeds(name: &str, size: i32) -> Result<Vec<Post>, Box<dyn Error>> {
    let mut start = true;
    let mut res = Vec::new() as Vec<Post>;
    loop {
        let url = if start {
            format!("https://{}.zhubai.love/api/publications/{}/posts?publication_id_type=token&limit=10", name, name)
        } else {
            format!("https://{}.zhubai.love/api/publications/{}/posts?publication_id_type=token&created_at={}&limit=10", name, name, res.last().unwrap().created_at)
        };
        start = false;
        let resp = reqwest::get(url).await?.json::<GetFeedsResp>().await?;
        let resp_len = resp.data.len();
        // push data to res
        res.extend(resp.data.into_iter().map(|post: Post| Post {
            site_name: String::from(name),
            url: post.url(name),
            ..post
        }).collect::<Vec<Post>>());
        if resp_len < 10 || res.len() >= size as usize {
            break;
        }
    }

    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
struct GetFeedsResp {
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
    pub publication: Publication,
    pub title: String,
    pub updated_at: i64,
    #[serde(skip_deserializing)]
    pub site_name: String,
    #[serde(skip_deserializing)]
    pub url: String,
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
pub struct Author {
    pub avatar: String,
    pub description: String,
    pub id: String,
    pub name: String,
}

impl Post {
    fn url(&self, site_name: &str) -> String {
        String::from(format!("https://{}.zhubai.love/posts/{}", site_name, self.id))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SiteInfo {
    pub author: Author,
    pub created_at: i64,
    pub description: String,
    pub id: String,
    pub name: String,
    // name
    pub token: String,
    pub updated_at: i64,
}

impl SiteInfo {
    pub fn url(&self) -> String {
        String::from(format!("https://{}.zhubai.love", self.name))
    }
}