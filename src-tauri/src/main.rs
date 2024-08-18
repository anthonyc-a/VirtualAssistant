// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysinfo::{Components, Disks, Networks, System};
use tauri::Manager;


#[derive(Serialize)]
struct CpuInfo {
    name: String,
    usage: f32,
    frequency: u64,
}

#[tauri::command]
fn get_cpu_info() -> Vec<CpuInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.cpus().iter().map(|cpu| CpuInfo {
        name: cpu.name().to_string(),
        usage: cpu.cpu_usage(),
        frequency: cpu.frequency(),
    }).collect()
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

fn main() {
    tauri::Builder::default()
        .setup(|app| {
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
        .invoke_handler(tauri::generate_handler![
            save_name,
            get_name,
            greet,
            bye,
            get_formatted_local_time,
            get_documents_files,
            get_cpu_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
