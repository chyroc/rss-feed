use axum::response::IntoResponse;

use crate::adapter::v2ex::rss::get_v2ex_all_rss;
use crate::httphandle::resp::RssResponse;

pub async fn get_feed() -> impl IntoResponse {
    match get_v2ex_all_rss().await {
        Ok(data) => RssResponse::Chan(data),
        Err(err) => RssResponse::Err(err)
    }
}
