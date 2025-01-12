use serde::Deserialize;
use std::env;
use crate::consts;

pub enum Env {
    Development,
    Production,
}

impl Env {
    pub fn from_env() -> Self {
        let env = env::var("APP_ENVIRONMENT");
        if env.is_ok() {
            let env = env.unwrap();
            Self::from_str(&env)
        } else {
            Self::from_str("")
        }
    }

    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "production" | "prod" => {
                println!("Using Production Environment.");
                Env::Production
            }
            "development" | "dev" => {
                println!("Using Development Environment.");
                Env::Development
            }
            env => {
                println!(
                    "Unknown Environment: \"{env}\", \
                    falling back to Development Environment."
                );
                Env::Development
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

#[derive(Debug, Deserialize)]
pub struct TracingConfig {
    pub max_level: LogLevel,
    pub file_name: bool,
    pub line_number: bool,
    pub thread_id: bool,
    pub event_target: bool,
    pub log_to_file: bool,
    pub log_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub tracing: TracingConfig,
}

impl LogLevel {
    pub fn to_level(&self) -> tracing::Level {
        match self {
            LogLevel::TRACE => tracing::Level::TRACE,
            LogLevel::DEBUG => tracing::Level::DEBUG,
            LogLevel::INFO => tracing::Level::INFO,
            LogLevel::WARN => tracing::Level::WARN,
            LogLevel::ERROR => tracing::Level::ERROR,
        }
    }
}

pub fn load_config() -> AppConfig {
    let config_path: &str;
    match Env::from_env() {
        Env::Development => config_path = consts::CONFIG_PATH_DEV,
        Env::Production => config_path = consts::CONFIG_PATH_PROD,
    }

    let f = std::fs::File::open(config_path)
        .expect(format!("Unable to open config file: {config_path}").as_str());

    let app_config: AppConfig = serde_yaml::from_reader(f).unwrap();
    app_config
}
