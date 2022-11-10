use std::error::Error;

use rss::{Channel, ChannelBuilder, GuidBuilder, Item, ItemBuilder};

use crate::adapter::github::trending;
use crate::adapter::github::trending::GithubRepo;

pub async fn get_github_trending_rss(lang: String, since: String) -> Result<Channel, Box<dyn Error>> {
    let site_title = format!("GitHub - Trending - {} - {}", lang, since);
    let feeds = match trending::get_github_trending(&lang, &since).await {
        Ok(data) => data,
        Err(err) => return Err(err)
    };

    Ok(ChannelBuilder::default()
        .title(site_title)
        .link("https://github.com")
        .items(feeds.into_iter().map(|feed: GithubRepo|
            ItemBuilder::default()
                .title(feed.title)
                .link(feed.link.clone())
                .description(feed.description)
                .guid(GuidBuilder::default().permalink(false).value(feed.link.to_string()).build())
                .build()
        ).collect::<Vec<Item>>())
        .build())
}
