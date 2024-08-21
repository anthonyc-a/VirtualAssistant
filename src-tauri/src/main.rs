// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod image_generator;
use image_generator::ImageGeneratorState;

mod claude;

mod pomodoro;
use pomodoro::{get_timer_state, pause_timer, reset_timer, start_timer, Timer, TimerState};

mod weather;

use weather::{get_weather_data, WeatherData};

use chrono::Local;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{Components, Disks, Networks, System};
use tauri::Manager;
use tauri::State;

#[derive(Debug, Serialize)]
struct H2Content {
    content: String,
}

#[derive(Serialize)]
struct CpuInfo {
    name: String,
    usage: f32,
    frequency: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileInfo {
    name: String,
    is_dir: bool,
    extension: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredName {
    name: String,
}

#[tauri::command]
fn get_weather(lat: f64, lon: f64) -> Result<WeatherData, String> {
    get_weather_data(lat, lon).map_err(|e| e.to_string())
}

#[tauri::command]
async fn ask_claude(prompt: String) -> Result<String, String> {
    claude::ask_claude(&prompt).await.map_err(|e| e.to_string())
}

#[tauri::command]
fn scrape_h2_content(url: String) -> Result<Vec<H2Content>, String> {
    let resp = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;
    let body = resp.text().map_err(|e| e.to_string())?;
    let document = Html::parse_document(&body);

    let h2_selector = Selector::parse("h2").unwrap();

    let h2_contents: Vec<H2Content> = document
        .select(&h2_selector)
        .map(|element| H2Content {
            content: element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string(),
        })
        .collect();

    Ok(h2_contents)
}

#[tauri::command]
fn get_cpu_info() -> Vec<CpuInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.cpus()
        .iter()
        .map(|cpu| CpuInfo {
            name: cpu.name().to_string(),
            usage: cpu.cpu_usage(),
            frequency: cpu.frequency(),
        })
        .collect()
}

#[tauri::command]
fn get_system_info(name: String) -> Result<(), String> {
    let stored_name = StoredName { name };
    let json = serde_json::to_string(&stored_name).map_err(|e| e.to_string())?;
    fs::write(get_storage_path(), json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn save_name(name: String) -> Result<(), String> {
    let stored_name = StoredName { name };
    let json = serde_json::to_string(&stored_name).map_err(|e| e.to_string())?;
    fs::write(get_storage_path(), json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_name() -> Result<String, String> {
    let path = get_storage_path();
    if !path.exists() {
        return Ok(String::new());
    }
    let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let stored_name: StoredName = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(stored_name.name)
}

fn get_storage_path() -> PathBuf {
    let mut path = tauri::api::path::app_data_dir(&tauri::Config::default())
        .expect("Failed to get app data dir");
    path.push("stored_name.json");
    path
}

#[tauri::command]
fn get_documents_files() -> Result<Vec<FileInfo>, String> {
    let mut documents_path = dirs::document_dir().ok_or("Failed to get Documents directory")?;
    documents_path.push("proj/si/landing"); // Optional: specify a subfolder in Documents

    let entries = fs::read_dir(documents_path).map_err(|e| e.to_string())?;

    let files: Vec<FileInfo> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let name = path.file_name()?.to_string_lossy().into_owned();
            let is_dir = path.is_dir();
            let extension = path.extension().map(|e| e.to_string_lossy().into_owned());

            Some(FileInfo {
                name,
                is_dir,
                extension,
            })
        })
        .collect();

    Ok(files)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn bye(name: &str) -> String {
    format!("Hello, {}! You've been greeted from The Other SIde!", name)
}

#[tauri::command]
fn get_formatted_local_time() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

struct AppState {
    client: Client,
}

#[tauri::command]
async fn send_message_to_anthropic(
    message: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let url = "https://api.anthropic.com/v1/messages";
    let api_key = "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"; // Replace with your actual API key
    let anthropic_version = "2023-06-01"; // This is the current version as of now, but check Anthropic's documentation for the latest version

    let body = json!({
        "model": "claude-3-opus-20240229",
        "max_tokens": 1024,
        "messages": [
            { "role": "user", "content": message }
        ]
    });

    let response = state
        .client
        .post(url)
        .header("x-api-key", api_key)
        .header("anthropic-version", anthropic_version)
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_body: Value = response.json().await.map_err(|e| e.to_string())?;
    Ok(serde_json::to_string_pretty(&response_body).unwrap())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            let app_handle = app.handle();
            let time = Arc::new(Mutex::new(String::new()));
            let time_clone = Arc::clone(&time);

            // Spawn a new thread to update time every second
            thread::spawn(move || loop {
                let current_time = get_formatted_local_time();
                *time_clone.lock().unwrap() = current_time.clone();
                app_handle.emit_all("time-update", current_time).unwrap();
                thread::sleep(Duration::from_secs(1));
            });

            Ok(())
        })
        .manage(AppState {
            client: Client::new(),
        })
        .manage(TimerState(Mutex::new(Timer::new())))
        .manage(ImageGeneratorState::new()) // Add this line
        .invoke_handler(tauri::generate_handler![
            save_name,
            get_name,
            greet,
            bye,
            get_formatted_local_time,
            get_documents_files,
            get_cpu_info,
            scrape_h2_content,
            start_timer,
            pause_timer,
            reset_timer,
            get_timer_state,
            get_weather,
            image_generator::generate_image,
            image_generator::get_image_urls,
            ask_claude,
            send_message_to_anthropic
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
