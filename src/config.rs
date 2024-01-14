use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub endpoints: Vec<EndpointConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Method {
    #[serde(alias = "GET")]
    Get,
    #[serde(alias = "POST")]
    Post,
    #[serde(alias = "PUT")]
    Put,
    #[serde(alias = "DELETE")]
    Delete,
    #[serde(alias = "PATCH")]
    Patch,
    #[serde(alias = "OPTIONS")]
    Options,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let method = match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
            Method::Options => "OPTIONS",
        };
        write!(f, "{}", method)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct EndpointConfig {
    pub name: String,
    pub url: String,
    pub method: Method,
    pub concurrent_requests: u64,
    pub cycles: u64,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Value>,
    pub expected_status: Option<u16>,
    pub expected_body: Option<Value>,
    pub expected_headers: Option<HashMap<String, String>>,
    pub threshold: Option<u128>,
}

impl Config {
    pub fn from_config_file(path: &str) -> Result<Self> {
        let contents = read_to_string(path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
