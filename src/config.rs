use crate::error::{AppError, Result};
use config::{Config as ConfigBuilder, Environment, File};
use serde::Deserialize;
use std::sync::Arc;

/// Application configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

impl Config {
    /// Load configuration from files and environment variables
    pub fn load() -> Result<Arc<Self>> {
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

        let config = ConfigBuilder::builder()
            // Load default configuration
            .add_source(File::with_name("config/default.toml").required(false))
            // Load environment-specific configuration
            .add_source(File::with_name(&format!("config/{}.toml", env)).required(false))
            // Load local configuration (for overrides, not committed to git)
            .add_source(File::with_name("config/local.toml").required(false))
            // Override with environment variables (with prefix APP_)
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()
            .map_err(|e| AppError::ConfigError(e.to_string()))?;

        let config: Config = config
            .try_deserialize()
            .map_err(|e| AppError::ConfigError(e.to_string()))?;

        Ok(Arc::new(config))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                environment: "development".to_string(),
            },
            database: DatabaseConfig {
                url: "postgres://postgres:postgres@localhost/rusty_saas".to_string(),
                max_connections: 10,
                min_connections: 2,
            },
            jwt: JwtConfig {
                secret: "CHANGE_THIS_SECRET_IN_PRODUCTION".to_string(),
                expiration_hours: 24,
            },
            cors: CorsConfig {
                allowed_origins: vec!["http://localhost:3000".to_string()],
            },
        }
    }
}
