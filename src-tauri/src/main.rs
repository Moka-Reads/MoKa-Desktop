// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use mokareads_core::resources::{Cacher, cheatsheet::Cheatsheet, Parser};
use reqwest::Response;
use serde::{Deserialize, Serialize};

const API_LANGMAP: &str = "/api/lang_map";

async fn get_api(url: &str) -> String{
    let url = format!("https://mokareads.org{url}");
    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await.unwrap();
    resp.text().await.unwrap()
}


#[tauri::command]
async fn fetch_cheatsheets() -> Result<String, tauri::Error>{
    let str = get_api(API_LANGMAP).await;
    Ok(str)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_cheatsheets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
