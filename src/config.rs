/*
 * @Author: lihuan
 * @Date: 2025-03-21 22:33:09
 * @LastEditors: lihuan
 * @LastEditTime: 2025-03-23 21:57:02
 * @Email: 17719495105@163.com
 */
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_exp: i64,
    pub refresh_exp: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub rate_limit: RateLimitConfig,
}
#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub capacity: usize,
    pub fill_rate: u64
}

impl AppConfig {

    pub fn load() -> Result<Self, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to get current dir");
        let config_dir = base_path.join("config");

        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
        
        config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.toml")))
            .add_source(config::File::from(config_dir.join(env)))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?
            .try_deserialize()
    }
}