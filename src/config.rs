use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::env;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherConfig {
    pub city: String,
    pub units: String,
    pub refresh_interval: u64,
    pub language: String,
}

impl Default for WeatherConfig {
    fn default() -> Self {
        WeatherConfig {
            city: "London".to_string(),
            units: "metric".to_string(),
            refresh_interval: 300,
            language: "en".to_string(),
        }
    }
}

pub fn find_config() -> Option<PathBuf> {
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let config_path = exe_dir.join("weather_config.toml");
            if config_path.exists() {
                return Some(config_path);
            }
        }
    }

    if let Ok(cwd) = env::current_dir() {
        let config_path = cwd.join("weather_config.toml");
        if config_path.exists() {
            return Some(config_path);
        }
    }

    if let Some(home_dir) = dirs::home_dir() {
        let config_path = home_dir
            .join(".config")
            .join("weather_plugin")
            .join("config.toml");
        
        if config_path.exists() {
            return Some(config_path);
        }
    }

    None
}

pub fn load_config() -> Result<WeatherConfig> {
    if let Some(config_path) = find_config() {
        let config_content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config at {}", config_path.display()))?;
        
        let config: WeatherConfig = toml::from_str(&config_content)
            .with_context(|| format!("Failed to parse config at {}", config_path.display()))?;
        
        return Ok(config);
    }

    let config = WeatherConfig::default();
    save_config(&config)?;
    Ok(config)
}

pub fn save_config(config: &WeatherConfig) -> Result<()> {
    let config_path = if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            exe_dir.join("weather_config.toml")
        } else {
            PathBuf::from("weather_config.toml")
        }
    } else {
        PathBuf::from("weather_config.toml")
    };

    let toml = toml::to_string(config)?;
    fs::write(&config_path, toml)
        .with_context(|| format!("Failed to write config to {}", config_path.display()))?;
    
    println!("Config created at: {}", config_path.display());
    Ok(())
}
