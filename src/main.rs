use axum::{
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router, routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use rss_feed::httphandle;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(httphandle::root::root))
        .route("/users", post(create_user));


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

// / use std::error::Error;
//
// use rss_feed::adapter::zhubai;
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let posts = zhubai::get_zhubai("rustbee").await?;
//     println!("posts.len: {}", posts.len());
//     for item in &posts {
//         println!("name: {}, title: {}, id: {}", item.author.name, item.title, item.id);
//     }
//
//     Ok(())
// }

