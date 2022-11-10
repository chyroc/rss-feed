use axum::response::IntoResponse;

use crate::adapter::gocn::rss::get_gocn_daily_rss;
use crate::httphandle::resp::RssResponse;

pub async fn get_daily() -> impl IntoResponse {
    match get_gocn_daily_rss().await {
        Ok(data) => RssResponse::Chan(data),
        Err(err) => RssResponse::Err(err)
    }
}
