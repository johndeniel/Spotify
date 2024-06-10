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
    let html_content = fs::read_to_string("src/static/index.html").unwrap();
    Html(html_content)
}
