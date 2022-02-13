use std::fs;
use std::fmt;

use serde_derive::Deserialize;
use log::info;

#[derive(Deserialize)]
pub struct Config {
    ip: String,
    port: u16,
}

impl Config {
    pub fn load(config_file: &str) -> Config {
        let contents = fs::read_to_string(config_file)
            .expect("Something went wrong reading the file");

        let config: Config = toml::from_str(contents.as_str()).unwrap();
        info!("Config file successfully read: {}" , config);

        config
    }
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config: ip={}, port={}", self.ip, self.port)
    }
}