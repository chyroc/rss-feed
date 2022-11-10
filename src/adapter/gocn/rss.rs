use std::error::Error;

use rss::{Channel, ChannelBuilder, GuidBuilder, Item, ItemBuilder};

use crate::adapter::gocn::daily::{get_feed, Post};

const SITE_NAME: &str = "GoCN - 每日新闻";
const SITE_URL: &str = "https://gocn.vip/topics/cate/18?page=1&grade=hot";

pub async fn get_gocn_daily_rss() -> Result<Channel, Box<dyn Error>> {
    let feeds = match get_feed().await {
        Ok(data) => data,
        Err(err) => return Err(err)
    };

    Ok(ChannelBuilder::default()
        .title(SITE_NAME)
        .link(SITE_URL)
        .items(feeds.into_iter().map(|feed: Post|
            ItemBuilder::default()
                .title(feed.title)
                .link(feed.url.to_string())
                .description(feed.content)
                .guid(GuidBuilder::default().permalink(false).value(feed.url.to_string()).build())
                .pub_date(feed.ctime.to_string())
                .author(feed.nickname)
                .build()
        ).collect::<Vec<Item>>())
        .build())
}
