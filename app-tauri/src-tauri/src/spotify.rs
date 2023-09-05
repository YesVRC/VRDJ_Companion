use std::collections::HashMap;
use reqwest::Client;
use crate::youtube::youtube_music_search;

#[tauri::command]
pub async fn spotify_auth() -> String{
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    let client_id = std::env::var("SPOTIFY_CLIENT").expect("Failed to do something idk");
    let client_secret = std::env::var("SPOTIFY_SECRET").expect("Failed to do something idk");
    let client: Client = Client::new();
    let req = client.post("https://accounts.spotify.com/api/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&params);
    let res = req.send().await.unwrap().text().await.unwrap();
    res
}

#[tauri::command]
pub async fn spotify_get_track(id: String, token: String) -> String {
    let client: Client = reqwest::Client::new();
    let req = client.get(format!("https://api.spotify.com/v1/tracks/{id}"))
        .bearer_auth(token);
    let res = req.send().await.unwrap().text().await.unwrap();
    res
}