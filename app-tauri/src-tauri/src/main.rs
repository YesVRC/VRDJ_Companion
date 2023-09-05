// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
mod spotify;
#[macro_use]
mod youtube;

use dotenv;
use reqwest;



fn main() {
  dotenv::dotenv().ok();


  tauri::Builder::default()
      // This is where you pass in your commands
      .invoke_handler(tauri::generate_handler![my_custom_command,network_test,
        spotify::spotify_auth,
        spotify::spotify_get_track,
        youtube::youtube_music_search,
        youtube::youtube_search,
        youtube::youtube_download
      ])
      .plugin(tauri_plugin_store::Builder::default().build())
      .run(tauri::generate_context!())
      .expect("failed to run app");
}

#[tauri::command]
fn my_custom_command() -> String {
  "Hello from Rust!".into()
}

#[tauri::command]
async fn network_test(url: String) -> String {
  let body = reqwest::get(url).await.unwrap().text().await.unwrap();
  body
}



