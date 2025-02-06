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
pub struct MySqlConfig  {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig  {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub mysql: MySqlConfig,
    pub redis: RedisConfig,
}

pub fn load_config() -> AppConfig {
    // set project root path to load .env file
    let project_root = env::current_dir().unwrap().join(".");
    println!("Current project root: {:?}", project_root);
    env::set_current_dir(&project_root).expect("Failed to set current directory");

    // load .env file
    dotenv().ok();
    let environment = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    println!("Environment: {}", environment);

    let mut config = Config::builder();
    let config_file = format!("api/src/resources/application-{}.toml", environment);
    println!("Config file path: {:?}", config_file);

    // load config file from resources
    config = config.add_source(File::with_name(&config_file));
    config.build()
        .expect("Failed to build configuration")
        .try_deserialize::<AppConfig>()
        .expect("Failed to deserialize configuration")
}
