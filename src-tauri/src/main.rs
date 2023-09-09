// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


async fn get_resources() -> String{
    let client = reqwest::Client::new();
    let resp = client.get("https://mokareads.org/download/resources").send().await.unwrap();
    resp.text().await.unwrap()
}


#[tauri::command]
async fn greet() -> Result<String, tauri::Error> {
    Ok(get_resources().await)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
