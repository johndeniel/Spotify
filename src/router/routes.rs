use axum::{routing::get, Router};
use axum::response::{Html, IntoResponse};
use crate::spotify::playlist;
use std::fs;

pub fn app() -> Router {
    Router::new()
        .route("/", get(serve_html))
        .route("/playlist", get(playlist::get_playlist_json))
}

async fn serve_html() -> impl IntoResponse {
    // For production in a Docker image, read the HTML file from the deployed location
    let html_content = fs::read_to_string("/usr/local/bin/static/index.html").unwrap();

    // For local development, read the HTML file from the source directory
    // let html_content = fs::read_to_string("src/static/index.html").unwrap();

    Html(html_content)
}
