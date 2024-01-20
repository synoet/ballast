use crate::config::{EndpointConfig, Method};
use anyhow::Result;
use futures::Future;
use reqwest::Client;
use serde_json::Value;
use std::{collections::HashMap, pin::Pin};

#[derive(Debug, Clone)]
pub struct RequestOutput {
    pub duration: u128,
    pub success: bool,
    pub status: u16,
    pub response_body: Option<Value>,
    pub response_headers: Option<HashMap<String, String>>,
}

pub type Request = Pin<Box<dyn Future<Output = RequestOutput> + Send>>;

pub struct TimedRequest {
    pub request: Request,
}

impl TimedRequest {
    pub fn from_config(client: &Client, config: &EndpointConfig) -> Result<Self> {
        let mut base_request = match config.method {
            Method::Get => client.get(&config.url),
            Method::Post => client.post(&config.url),
            Method::Put => client.put(&config.url),
            Method::Delete => client.delete(&config.url),
            Method::Patch => client.patch(&config.url),
            _ => {
                return Err(anyhow::anyhow!("Invalid HTTP method: {}", config.method));
            }
        };

        if let Some(headers) = &config.headers {
            for (key, value) in headers {
                base_request = base_request.header(key, value);
            }
        }

        if let Some(body) = &config.body {
            base_request = base_request.body(body.to_string());
        }

        let request = async move {
            let start = std::time::Instant::now();
            let response = base_request.send().await;
            let duration = start.elapsed().as_millis();
            match response {
                Ok(r) => {
                    let status = r.status().as_u16();
                    let headers: Option<HashMap<String, String>> =
                        match r.headers().iter().count() > 0 {
                            true => {
                                Some(HashMap::from_iter(r.headers().iter().map(|(k, v)| {
                                    (k.to_string(), v.to_str().unwrap().to_string())
                                })))
                            }
                            false => None,
                        };
                    let json: Option<Value> = r.json().await.unwrap_or(None);
                    RequestOutput {
                        duration,
                        success: true,
                        status,
                        response_body: json,
                        response_headers: headers,
                    }
                }
                Err(_e) => RequestOutput {
                    duration,
                    success: false,
                    status: 0,
                    response_body: None,
                    response_headers: None,
                },
            }
        };

        return Ok(TimedRequest {
            request: Box::pin(request),
        });
    }
}
