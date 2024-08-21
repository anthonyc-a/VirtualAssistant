use reqwest::Client;
use serde_json::{json, Value};
use tauri::Manager;
use std::sync::Mutex;

pub struct ImageGeneratorState {
    image_urls: Mutex<Vec<String>>,
}

impl ImageGeneratorState {
    pub fn new() -> Self {
        Self {
            image_urls: Mutex::new(Vec::new()),
        }
    }
}

#[tauri::command]
pub async fn generate_image(prompt: String, app_handle: tauri::AppHandle, state: tauri::State<'_, ImageGeneratorState>) -> Result<Value, String> {
    let client = Client::new();
    let fal_key = "d2e200a2-6d11-46ee-bc2a-a7dd34b4924a:dedb9dbc6c5aae03f14245162ef5d30f";

    let response = client
        .post("https://fal.run/fal-ai/flux-realism")
        .header("Authorization", format!("Key {}", fal_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "prompt": prompt,
            "enable_safety_checker": false
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    let response_text = response.text().await.map_err(|e| e.to_string())?;

    let response_json: Value = serde_json::from_str(&response_text)
        .unwrap_or_else(|_| json!({ "raw_response": response_text }));

    // Extract image URL from the response
    let image_url = response_json["image"].as_str().unwrap_or("").to_string();

    if !image_url.is_empty() {
        let mut image_urls = state.image_urls.lock().unwrap();
        image_urls.push(image_url.clone());
    }

    let result = json!({
        "status": status.as_u16(),
        "body": response_json,
        "image_urls": state.image_urls.lock().unwrap().clone()
    });

    // Emit an event to the frontend with the full response and all image URLs
    app_handle.emit_all("image_generated", &result).map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub fn get_image_urls(state: tauri::State<'_, ImageGeneratorState>) -> Vec<String> {
    state.image_urls.lock().unwrap().clone()
}