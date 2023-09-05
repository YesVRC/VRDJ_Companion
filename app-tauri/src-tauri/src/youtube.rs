use std::collections::HashMap;
use reqwest::Client;
use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use tauri::Manager;

#[tauri::command]
pub async fn youtube_music_search(search: String) -> String{
    let json_data = format!(r#"{{
    "context": {{
        "client": {{
            "clientName": "WEB_REMIX",
            "clientVersion": "1.20230821.01.01"

        }}
    }},
    "query": "{search}"
}}"#);
    let client: Client = Client::new();
    let req = client.post("https://music.youtube.com/youtubei/v1/search")
        .json(&json_data.to_owned());
    let res = req.send().await.unwrap().text().await.unwrap();
    res
}

#[tauri::command]
pub async fn youtube_search(search: String) -> String {
    let client: Client = Client::new();
    let mut params = HashMap::new();
    let key = std::env::var("YOUTUBE_KEY").expect("An error occurred getting youtube api key");
    params.insert("q", &search);
    params.insert("key", &key);
    let req = client.get("https://www.googleapis.com/youtube/v3/search")
        .query(&params);
    let res = req.send().await.unwrap().text().await.unwrap();
    res
}

#[tauri::command]
pub async fn youtube_download(url: String, fileName: String, app: tauri::AppHandle) -> String {
    let video_options = VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    let video = Video::new_with_options(url, video_options).unwrap();
    // Or direct download to path
    let path = std::path::Path::new(&fileName);

    let stream = video.stream().await.unwrap();
    let name = &fileName.split('.').collect::<Vec<_>>()[0];

    while let Some(chunk) = stream.chunk().await.unwrap() {
        println!("{}", &name);
        println!("{:#?}", chunk.len());
        // Do what you want with chunks
        // app.emit_all(&name, chunk).expect("TODO: panic message");
    }

    video.download(path).await.unwrap();
    path.to_owned().to_str().unwrap().to_string()
}