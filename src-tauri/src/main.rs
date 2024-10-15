// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use tauri::Manager;
use puppeteer::{Browser, LaunchOptions};

// Tauriのコマンド
#[tauri::command]
async fn save_page_content(url: String, folder_name: String) -> Result<String, String> {
    let launch_options = LaunchOptions::default().headless(true);
    let browser = Browser::launch(launch_options).await.map_err(|e| e.to_string())?;
    let page = browser.new_page().await.map_err(|e| e.to_string())?;

    // 指定されたURLにアクセス
    page.goto(&url, None).await.map_err(|e| e.to_string())?;

    // ページ内容を取得
    let content = page.content().await.map_err(|e| e.to_string())?;

    // 保存先フォルダを作成
    let app_data_dir = tauri::api::path::app_data_dir().expect("Could not get app data dir");
    let folder_path = app_data_dir.join(folder_name);
    fs::create_dir_all(&folder_path).map_err(|e| e.to_string())?;

    // ページ内容をHTMLファイルとして保存
    let file_path = folder_path.join("page.html");
    fs::write(&file_path, content).map_err(|e| e.to_string())?;

    // ブラウザを閉じる
    browser.close().await.map_err(|e| e.to_string())?;

    Ok(format!("ページが {} に保存されました。", file_path.display()))
}
// Tauriのメイン関数
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_page_content])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
