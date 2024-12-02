use config::{Config, File};
use std::env;
use serde::Deserialize;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig  {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

pub fn load_config() -> AppConfig {
    // load .env file
    dotenv().ok();
    let current_dir = env::current_dir().unwrap();
    let environment = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    println!("Environment: {}", environment);

    let mut config = Config::builder();
    let config_file = format!("api/src/resources/application-{}.toml", environment);
    println!("Config file path: {:?}", config_file);

    // load config file from resources
    config = config.add_source(File::with_name(&config_file));
    config.build().unwrap().try_deserialize::<AppConfig>().unwrap()
}
