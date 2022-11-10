use axum::response::IntoResponse;
use rss::{ChannelBuilder, GuidBuilder, Item, ItemBuilder};
use serde::{Deserialize, Serialize};

use crate::adapter::v2ex::{Feed, get_feeds};
use crate::httphandle::resp::RssResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    name: String,
}

const SITE_NAME: &str = "V2EX - 全部主题";
const SITE_URL: &str = "https://www.v2ex.com/?tab=all";
const SITE_DESCRIPTION: &str = "V2EX 的使命是为创意工作者打造一个最好的社区，他们在这里可以获得灵感，分享想法，找到伙伴，获得通向目标的加速度。
V2EX's mission is to create the best place for creative people. A place where they can get inspired, sharing ideas, finding partners, and gaining momentum on goals.";

pub async fn get_feed() -> impl IntoResponse {
    let feeds = match get_feeds().await {
        Ok(data) => data,
        Err(err) => return RssResponse::Err(err)
    };

    let items = feeds.into_iter().map(|feed: Feed|
        ItemBuilder::default()
            .title(format!("{} - {}", feed.title, feed.node.title))
            .link(feed.url.clone())
            .description(feed.content_rendered)
            .guid(GuidBuilder::default().permalink(false).value(feed.url.to_string()).build())
            .pub_date(feed.created.to_string())
            .build()
    ).collect::<Vec<Item>>();


    let channel = ChannelBuilder::default()
        .title(SITE_NAME)
        .link(SITE_URL)
        .description(SITE_DESCRIPTION)
        .items(items)
        .build();

    RssResponse::Chan(channel)
}
