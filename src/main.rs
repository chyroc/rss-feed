use std::net::SocketAddr;

use axum::{
    Router,
    routing::get,
};

use rss_feed::httphandle;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(httphandle::api_root::index))
        .route("/feed/zhubai/:name", get(httphandle::api_zhubai::get_user_posts))
        .route("/feed/v2ex/all", get(httphandle::api_v2ex::get_all))
        .route("/feed/gocn/daily", get(httphandle::api_gocn::get_daily))
        .route("/feed/github/trending/:lang/:since", get(httphandle::api_github::get_trending));


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

