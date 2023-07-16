use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::read_to_string;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub defaults: Option<DefaultsConfig>,
    pub endpoints: Vec<EndpointConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EndpointConfig {
    pub name: String,
    pub url: String,
    pub method: String,
    pub concurrent_requests: u64,
    pub cycles: u64,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
    pub expected_status: Option<u16>,
}

#[derive(Deserialize, Debug)]
pub struct DefaultsConfig {
    pub concurrent_requests: Option<usize>,
    pub cycles: Option<usize>,
    pub headers: Option<HashMap<String, String>>,
    pub expected_status: Option<u16>,
}

impl Config {
    pub fn from_config_file(path: &str) -> Result<Self> {
        let contents = read_to_string(path)
            .context("No ballast.toml config file found in current directory")?;
        let config: Config =
            toml::from_str(&contents).context("Invalid ballast.toml config file")?;
        Ok(config)
    }
}
