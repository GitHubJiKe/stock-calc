use crate::error::Result;
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub api: ApiConfig,
    pub display: DisplayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub default_interval: u64,
    pub enable_notifications: bool,
    pub enable_sound: bool,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub timeout: u64,
    pub retry_count: u32,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub color_output: bool,
    pub show_progress: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                default_interval: 60,
                enable_notifications: true,
                enable_sound: true,
                log_level: "info".to_string(),
            },
            api: ApiConfig {
                timeout: 8,
                retry_count: 3,
                user_agent: "StockCalc/1.0".to_string(),
            },
            display: DisplayConfig {
                color_output: true,
                show_progress: true,
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        let config = Config::builder()
            .add_source(File::from(config_path.as_path()).required(false))
            .add_source(Environment::with_prefix("STOCK_CALC"))
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;
        Ok(app_config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        
        // 确保配置目录存在
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let config_str = toml::to_string(self)
            .map_err(|e| crate::error::StockCalcError::ParseError(e.to_string()))?;
        
        std::fs::write(config_path, config_str)?;
        Ok(())
    }

    pub fn get_config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| crate::error::StockCalcError::ConfigError(
                config::ConfigError::NotFound("home directory".to_string())
            ))?;
        
        Ok(home_dir.join(".config").join("stock-calc").join("config.toml"))
    }

    pub fn get_data_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| crate::error::StockCalcError::ConfigError(
                config::ConfigError::NotFound("home directory".to_string())
            ))?;
        
        Ok(home_dir.join(".config").join("stock-calc").join("data.json"))
    }

    pub fn create_default_config() -> Result<()> {
        let config = AppConfig::default();
        config.save()
    }
} 