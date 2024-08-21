// src-tauri/src/claude.rs

use reqwest::Client;
use serde_json::{json, Value};
use anyhow::Result;

pub struct ClaudeClient {
    api_key: String,
    client: Client,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        ClaudeClient {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn complete(&self, prompt: &str) -> Result<String> {
        let url = "https://api.anthropic.com/v1/messages";
        let body = json!({
            "model": "claude-3-5-sonnet-20240620",
            "prompt": prompt,
            "max_tokens": 1024,
            "temperature": 0.7,
        });

        let response = self.client
            .post(url)
            .header("X-API-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let response_body: Value = response.json().await?;
        Ok(response_body["completion"].as_str().unwrap_or("").to_string())
    }
}

pub async fn ask_claude(prompt: &str) -> Result<String> {
    let api_key = "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".to_string(); // Replace with your actual API key
    let client = ClaudeClient::new(api_key);
    client.complete(prompt).await
}