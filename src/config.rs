use std::fs;
use std::fmt;

use serde_derive::Deserialize;
use log::{info, error};

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub threadpool: ThreadPoolConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    ip: String,
    port: u16,
}

#[derive(Deserialize)]
pub struct ThreadPoolConfig {
    pub workers: usize,
}

impl Config {
    pub fn load(config_file: &str) -> Config {

        let contents = fs::read_to_string(config_file)
        .map_err(|e| error!("{} reading failed with: {}", config_file, e)).unwrap();

        let config: Config = toml::from_str(contents.as_str())
        .map_err(|e| error!("{} parsing failed with: {}", config_file, e)).unwrap();

        info!("Config file successfully read: {}" , config);

        config
    }
}

impl ServerConfig {
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nServer: ip={}, port={}\nThreadPool: workers={}", self.server.ip, self.server.port, self.threadpool.workers)
    }
}