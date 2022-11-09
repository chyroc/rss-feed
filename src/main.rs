use std::error::Error;

use rss_feed::adapter::zhubai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let posts = zhubai::get_zhubai("rustbee").await?;
    println!("posts.len: {}", posts.len());
    for item in &posts {
        println!("name: {}, title: {}, id: {}", item.author.name, item.title, item.id);
    }

    Ok(())
}

