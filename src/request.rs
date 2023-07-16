use crate::config::EndpointConfig;
use anyhow::Result;
use futures::Future;
use reqwest::Client;
use std::pin::Pin;

#[derive(Debug)]
pub struct RequestOutput {
    pub duration: u128,
    pub success: bool,
    pub status: u16,
}

pub struct TimedRequest {
    pub request: Pin<Box<dyn Future<Output = RequestOutput> + Send>>,
}

impl TimedRequest {
    pub async fn from_config(client: &Client, config: &EndpointConfig) -> Result<Self> {
        let mut base_request = match config.method.as_str() {
            "GET" => client.get(&config.url),
            "POST" => client.post(&config.url),
            "PUT" => client.put(&config.url),
            "DELETE" => client.delete(&config.url),
            "HEAD" => client.head(&config.url),
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
            base_request = base_request.body(body.clone());
        }

        let request = async move {
            let start = std::time::Instant::now();
            let response = base_request.send().await;
            let duration = start.elapsed().as_millis();

            RequestOutput {
                duration,
                success: response.is_ok(),
                status: response
                    .as_ref()
                    .map(|r| r.status().as_u16())
                    .unwrap_or(200),
            }
        };

        return Ok(TimedRequest {
            request: Box::pin(request),
        });
    }
}
