use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use reqwest::blocking::Client;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub current: CurrentWeather,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentWeather {
    pub temp: f32,
    pub feels_like: f32,
    pub humidity: u8,
    pub wind_speed: f32,
    pub weather: Vec<WeatherDescription>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherDescription {
    pub main: String,
    pub description: String,
}

pub fn get_weather_data(lat: f64, lon: f64) -> Result<WeatherData> {
    dotenv::dotenv().ok();
    let api_key = env::var("OPENWEATHERMAP_API_KEY").context("API key not found")?;
    let url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&exclude=hourly,daily&appid={}&units=metric",
        lat, lon, api_key
    );

    let weather_data: WeatherData = Client::new()
        .get(&url)
        .send()
        .context("Failed to send request")?
        .json()
        .context("Failed to parse JSON")?;

    Ok(weather_data)
}