use axum::extract::Path;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::adapter::github::rss::get_github_trending_rss;
use crate::httphandle::resp::RssResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    lang: String,
    since: String,
}

pub async fn get_trending(Path(params): Path<Params>) -> impl IntoResponse {
    match get_github_trending_rss(params.lang, params.since).await {
        Ok(data) => RssResponse::Chan(data),
        Err(err) => RssResponse::Err(err)
    }
}
