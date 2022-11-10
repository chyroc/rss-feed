use axum::{
    extract::Path,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::adapter::zhubai::rss::get_zhubai_user_rss;
use crate::httphandle::resp::RssResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    name: String,
}

pub async fn get_feed(Path(params): Path<Params>) -> impl IntoResponse {
    match get_zhubai_user_rss(&params.name).await {
        Ok(data) => RssResponse::Chan(data),
        Err(err) => RssResponse::Err(err)
    }
}
