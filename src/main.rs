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
        .route("/", get(httphandle::root::index))
        .route("/feed/zhubai/:name", get(httphandle::zhubai::get_feed))
        .route("/feed/v2ex/all", get(httphandle::v2ex::get_feed));


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

