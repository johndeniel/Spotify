use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;

#[derive(Serialize, Deserialize)]
struct Playlist {
    name: String,
    tracks: Tracks,
}

#[derive(Serialize, Deserialize)]
struct Tracks {
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
struct Item {
    track: Track,
}

#[derive(Serialize, Deserialize)]
struct Track {
    name: String,
    artists: Vec<Artist>,
    album: Album,
    duration_ms: u64,
    popularity: u8,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize)]
struct Artist {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Album {
    name: String,
    images: Vec<Image>,
    release_date: String,
}

#[derive(Serialize, Deserialize)]
struct Image {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

/// Retrieves a Spotify access token using client credentials.
async fn get_spotify_token() -> String {
    dotenv().ok();
    let client_id = std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set");
    let client_secret = std::env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET not set");

    let client = reqwest::Client::new();

    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
    ];

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await
        .unwrap()
        .json::<TokenResponse>()
        .await
        .unwrap();

    response.access_token
}

/// Retrieves a playlist from Spotify using a provided playlist ID.
async fn get_playlist() -> Playlist {
    let token = get_spotify_token().await;
    let playlist_id = std::env::var("SPOTIFY_PLAYLIST_ID").expect("SPOTIFY_PLAYLIST_ID not set");

    let client = reqwest::Client::new();
    let url = format!("https://api.spotify.com/v1/playlists/{}", playlist_id);

    let response = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .unwrap()
        .json::<Playlist>()
        .await
        .unwrap();

    response
}

/// Retrieves a playlist and returns it as JSON.
pub async fn get_playlist_json() -> impl IntoResponse {
    let playlist = get_playlist().await;
    Json(playlist)
}
