use axum::{
    extract::Path,
    response::IntoResponse,
};
use rss::{ChannelBuilder, Guid, Item, ItemBuilder};
use serde::{Deserialize, Serialize};

use crate::adapter::zhubai;
use crate::httphandle::resp::RssResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    name: String,
}

const SITE_NAME: &str = "竹白";

pub async fn get_feed(Path(params): Path<Params>) -> impl IntoResponse {
    let site_info = match zhubai::get_info(&params.name).await {
        Ok(data) => data,
        Err(err) => return RssResponse::Err(err)
    };
    let posts = match zhubai::get_feeds(&params.name, 20).await {
        Ok(data) => data,
        Err(err) => return RssResponse::Err(err)
    };

    let mut items = Vec::new() as Vec<Item>;
    for post in &posts {
        items.push(ItemBuilder::default()
            .title(post.title.to_string())
            .link(post.url.to_string())
            .author(post.author.name.to_string())
            .guid(Guid {
                value: post.url.to_string(),
                permalink: false,
            })
            .pub_date(post.created_at.to_string())
            .build())
    }

    let channel = ChannelBuilder::default()
        .title(format!("{} - {}", SITE_NAME, site_info.name))
        .link(site_info.url())
        .description(site_info.description)
        .pub_date(site_info.created_at.to_string())
        .items(items)
        .build();

    RssResponse::Chan(channel)
}
