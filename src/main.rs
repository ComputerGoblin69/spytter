#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod api;

use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let state = Arc::new(api::SpytterState::new());

    let app = Router::new()
        .route_service("/", ServeFile::new("static/index.html"))
        .route("/api/spyyts", get(api::spyyts).post(api::post_spyyt))
        .fallback_service(
            ServeDir::new("static")
                .not_found_service(ServeFile::new("static/404.html")),
        )
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
