use crate::config::config;
use crate::proxy::ServiceType;
use anyhow::Result;
use clap::builder::Str;
use log::{debug, info};
use pingora::{Error, ReadError};
use pingora_core::prelude::Opt;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Configs {
    server: Vec<Server>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    protocol: String,
    address: String,
    upstream: Vec<String>,
    location: Option<Vec<Location>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    pub match_path: String,
    pub proxy_host: Option<String>,
    pub proxy_url: Option<String>,
}

/// 读取pingora配置文件
#[derive(Debug, Deserialize, Clone)]
struct PingoraConfig {
    pub server_path: String,
}

pub fn get_config_struct(file_path: String) -> Result<Configs> {
    info!("file_path: {}", file_path);

    let mut file = File::open(file_path.as_str())
        .expect(format!("Failed to open config file path: {}", file_path).as_str());
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config file");
    // 解析YAML配置文件
    let config: Configs =
        serde_yaml::from_str(&contents).expect("Configs Failed to parse config file");

    // 使用配置
    info!("config: {:?}", config);
    Ok(config)
}

pub fn get_pingora_struct(file_path: String) -> String {
    info!("file_path: {}", file_path);

    let mut file = File::open(file_path.as_str())
        .expect(format!("Failed to open config file path: {}", file_path).as_str());
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config file");

    // 解析YAML配置文件
    let pingoraConfig: PingoraConfig =
        serde_yaml::from_str(&contents).expect("PingoraConfig Failed to parse config file");
    pingoraConfig.server_path
}

impl Configs {
    pub fn get_server(&self) -> Vec<Server> {
        self.server.clone()
    }

    pub fn load_yaml_with_opt_override(path: Option<String>) -> Result<Self> {
        if let Some(path) = path {
            let mut conf = Self::load_from_yaml(path)?;
            Ok(conf)
        } else {
            Err(anyhow::anyhow!("division by zero"))
        }
    }

    pub fn load_from_yaml<P>(config_file_path: P) -> Result<Self>
    where
        P: std::fmt::Display,
    {
        let config_file_path = get_pingora_struct(config_file_path.to_string());
        get_config_struct(config_file_path)
    }
}

impl Server {
    pub fn get_protocol(&self) -> ServiceType {
        match self.protocol.as_str() {
            "http" => ServiceType::Http,
            "tcp" => ServiceType::Tcp,
            _ => ServiceType::None,
        }
    }

    pub fn get_address(&self) -> &str {
        self.address.as_str()
    }

    pub fn get_upstream(&self) -> Vec<String> {
        self.upstream.clone()
    }

    pub fn get_location(&self) -> Option<Vec<Location>> {
        self.location.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::config::config::Configs;
    use std::fs::File;
    use std::io::Read;

    #[test]
    pub fn test_read_config() {
        let mut file =
            File::open("/Users/dragon/dragon/word/rustfile/load_balancer/src/config/config.yaml")
                .expect("Failed to open config file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read config file");

        // 解析YAML配置文件
        let config: Configs = serde_yaml::from_str(&contents).expect("Failed to parse config file");

        // 使用配置
        println!("{:?}", config);
    }
}
