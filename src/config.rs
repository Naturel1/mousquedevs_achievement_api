use std::env;

/// Global application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/maadb".to_string()),
            host: env::var("ROCKET_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("ROCKET_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8000),
        }
    }
}
