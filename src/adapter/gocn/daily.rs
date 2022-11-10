use std::error::Error;
use std::io::{Error as IOError, ErrorKind};

use futures::future;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

pub async fn get_feed() -> Result<Vec<Post>, Box<dyn Error>> {
    let url = "https://gocn.vip/apiv3/topic/list?currentPage=1&cate2Id=18&grade=new";
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header(USER_AGENT, "My Rust Program 1.0")
        .send()
        .await?.json::<GetListResp>().await?;
    // println!("{}", resp);

    // let resp = reqwest::get(url).await?.json::<GetListResp>().await?;
    if resp.code != 0 {
        return Err(Box::new(IOError::new(ErrorKind::Other, format!("gocn.vip api error: {}", resp.msg))));
    }

    let contents = future::join_all(resp.data.list.iter().map(|post: &Post| get_post_content(&post.guid))).await as Vec<String>;

    let posts = resp.data.list.into_iter().enumerate().map(|(idx, post): (usize, Post)|
        Post {
            url: format!("https://gocn.vip/topics/{}", post.guid),
            content: contents[idx].to_string(),
            ..post
        }
    ).collect::<Vec<Post>>();

    Ok(posts)
}

async fn get_post_content(guid: &str) -> String {
    let url = format!("https://gocn.vip/apiv3/topic/{}/info", guid);
    match reqwest::get(url).await {
        Ok(resp) => {
            match resp.json::<GetPostContentResp>().await {
                Ok(resp) => resp.data.topic.content_html,
                Err(_) => String::new()
            }
        }
        Err(_) => String::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub total: i64,
    #[serde(rename = "currentPage")]
    pub current_page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub sort: String,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub guid: String,
    pub uid: i64,
    pub nickname: String,
    pub avatar: String,
    pub title: String,
    pub summary: Option<String>,
    pub ctime: i64,
    #[serde(rename = "cntView")]
    pub cnt_view: i64,
    #[serde(rename = "cate2Id")]
    pub cate2id: i64,
    #[serde(rename = "cate2Title")]
    pub cate2title: String,
    #[serde(rename = "cntReply")]
    pub cnt_reply: Option<i64>,
    #[serde(rename = "cntLike")]
    pub cnt_like: Option<i64>,
    #[serde(skip_deserializing)]
    pub content: String,
    #[serde(skip_deserializing)]
    pub url: String,
}

#[derive(Serialize, Deserialize)]
struct GetListRespData {
    pub list: Vec<Post>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
struct GetListResp {
    pub code: i64,
    pub msg: String,
    pub data: GetListRespData,
}


#[derive(Serialize, Deserialize)]
struct GetPostContentRespDataTopic {
    #[serde(rename = "contentHtml")]
    pub content_html: String,
}

#[derive(Serialize, Deserialize)]
struct GetPostContentRespData {
    pub topic: GetPostContentRespDataTopic,
}

#[derive(Serialize, Deserialize)]
struct GetPostContentResp {
    pub code: i64,
    pub msg: String,
    pub data: GetPostContentRespData,
}