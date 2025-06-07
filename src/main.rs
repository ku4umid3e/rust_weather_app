use reqwest;
use serde::Deserialize;
use std::io;

#[derive(Debug, Deserialize)]
struct WeatherData {
    current_condition: Vec<CurrentCondition>,
}

#[derive(Debug, Deserialize)]
struct CurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: String,
    humidity: String,
    #[serde(rename = "windspeedKmph")]
    windspeed_kmph: String,
    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<WeatherDesc>,
}

#[derive(Debug, Deserialize)]
struct WeatherDesc {
    value: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Please enter city for weather");
    let mut city = String::new();
    io::stdin()
        .read_line(&mut city)
        .expect("Error reading stdin.");
    let city = city.trim();

    let url = format!("https://wttr.in/{}?format=j1", city);

    let response= reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        eprintln!("Error: server return status {}", response.status());
        return Ok(());
}

let weather: WeatherData = response.json()?;

if let Some(condition) = weather.current_condition.first() {
    if let Some(description) = condition.weather_desc.first() {
        println!("\n--- Weather in {} ---", city);
        println!("Temperature: {} C", condition.temp_c);
        println!("Описание: {}", description.value);
        println!("Влажность: {}%", condition.humidity);
        println!("Скорость ветра: {} км/ч", condition.windspeed_kmph);
    }
} else {
    println!("Данные о погоде не найдены");
}

Ok(())
}

