use std::error::Error;

use rss::{Channel, ChannelBuilder, Guid, ItemBuilder};

use crate::adapter::zhubai::user;
use crate::adapter::zhubai::user::Post;

const SITE_NAME: &str = "竹白";

pub async fn get_zhubai_user_posts_rss(name: &str) -> Result<Channel, Box<dyn Error>> {
    let site_info = match user::get_info(name).await {
        Ok(data) => data,
        Err(err) => return Err(err)
    };
    let posts = match user::get_feeds(name, 20).await {
        Ok(data) => data,
        Err(err) => return Err(err)
    };

    Ok(ChannelBuilder::default()
        .title(format!("{} - {}", SITE_NAME, site_info.name))
        .link(site_info.url())
        .description(site_info.description)
        .pub_date(site_info.created_at.to_string())
        .items(posts.into_iter().map(|post: Post|
            ItemBuilder::default()
                .title(post.title.to_string())
                .link(post.url.to_string())
                .author(post.author.name.to_string())
                .guid(Guid {
                    value: post.url.to_string(),
                    permalink: false,
                })
                .pub_date(post.created_at.to_string())
                .build())
            .collect::<Vec<_>>()
        )
        .build())
}
