use std::error::Error;

use axum::response::{IntoResponse, Response};
use rss::Channel;

pub enum RssResponse {
    Chan(Channel),
    Err(Box<dyn Error>),
}

impl IntoResponse for RssResponse {
    fn into_response(self) -> Response {
        match self {
            RssResponse::Chan(channel) => {
                let xml = channel.to_string();
                Response::builder()
                    .header("Content-Type", "application/rss+xml; charset=utf-8")
                    .body(xml)
                    .unwrap().into_response()
            }
            RssResponse::Err(err) => {
                let err = format!("Error: {}", err);
                Response::builder()
                    .status(500)
                    .body(err)
                    .unwrap().into_response()
            }
        }
    }
}