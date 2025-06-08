mod config;

use anyhow::Result;
use config::load_config;
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WeatherData {
    current_condition: Vec<CurrentCondition>,
}

#[derive(Debug, Deserialize)]
struct CurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: String,
    #[serde(rename = "weatherCode")]
    weather_code: String,
}

fn get_wether_icon(code: &str) -> &str {
    match code {
        "113" => "☀️",
        "116" => "⛅️",
        "119" => "☁️",
        "122" => "☁️",
        "143" => "🌫",
        "176" => "🌦",
        "179" => "🌧",
        "182" => "🌧",
        "185" => "🌧",
        "200" => "⛈",
        "227" => "🌨",
        "230" => "❄️",
        "248" => "🌫",
        "260" => "🌫",
        "263" => "🌦",
        "266" => "🌦",
        "281" => "🌧",
        "284" => "🌧",
        "293" => "🌦",
        "296" => "🌦",
        "299" => "🌧",
        "302" => "🌧",
        "305" => "🌧",
        "308" => "🌧",
        "311" => "🌧",
        "314" => "🌧",
        "317" => "🌧",
        "320" => "🌨",
        "323" => "🌨",
        "326" => "🌨",
        "329" => "❄️",
        "332" => "❄️",
        "335" => "❄️",
        "338" => "❄️",
        "350" => "🌧",
        "353" => "🌦",
        "356" => "🌧",
        "359" => "🌧",
        "362" => "🌧",
        "365" => "🌧",
        "368" => "🌨",
        "371" => "❄️",
        "374" => "🌧",
        "377" => "🌧",
        "386" => "⛈",
        "389" => "🌩",
        "392" => "⛈",
        "395" => "❄️",
        _ => "�", // или другое значение по умолчанию
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let city = &config.city;

    let url = format!(
        "https://wttr.in/{}?format=j1&lang={}",
        city, config.language
    );

    let response= reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        eprintln!("Error: server return status {}", response.status());
        return Ok(());
}

let weather: WeatherData = response.json()?;

if let Some(condition) = weather.current_condition.first() {
    let icon = get_wether_icon(&condition.weather_code);
    println!("{}°C {}", condition.temp_c, icon);
} else {
    println!("Weather data not found.");
}

Ok(())
}

