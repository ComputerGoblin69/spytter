#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route_service("/", ServeFile::new("static/index.html"))
        .fallback_service(
            ServeDir::new("static")
                .not_found_service(ServeFile::new("static/404.html")),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
