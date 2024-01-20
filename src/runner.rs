use anyhow::Result;
use futures::future::join_all;
use std::pin::Pin;
use std::vec;
use tokio::time;
use tokio::time::Duration;
use tokio_stream::{self as stream, StreamExt};

use crate::config::{Config, EndpointConfig};
use crate::printer::Printer;
use crate::request::{Request, RequestOutput, TimedRequest};

pub struct Runner {
    config: Config,
    client: reqwest::Client,
}

pub type SingleCycle = Vec<RequestOutput>;

#[derive(Debug)]
pub struct SingleLoad {
    pub cycles: Vec<SingleCycle>,
    pub num_cycles: u64,
    pub num_concurrent_requests: u64,
    pub endpoint_name: String,
    pub endpoint_url: String,
}

pub type Loads = Vec<SingleLoad>;

impl Runner {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::new();
        Self { config, client }
    }

    fn make_request(&self, endpoint: &EndpointConfig) -> Result<Request> {
        Ok(TimedRequest::from_config(&self.client, &endpoint)?.request)
    }

    fn make_cycle(&self, endpoint: &EndpointConfig) -> Result<Vec<Request>> {
        Ok((0..endpoint.concurrent_requests)
            .map(|_| self.make_request(endpoint).unwrap())
            .collect::<Vec<_>>())
    }

    async fn log_ramp(&self, endpoint: &EndpointConfig) -> Result<()> {
        let max_count = (endpoint.cycles as f64 / 2.).ceil() as u64;
        let log_scale = (endpoint.concurrent_requests as f64).ln();

        let ramp = (0..max_count)
            .map(|i| {
                let curr_scale = i as f64 / max_count as f64 * log_scale;
                let num_requests = curr_scale.exp().ceil() as usize;
                let cycle = self.make_cycle(endpoint).unwrap();
                let cycle = cycle.into_iter().take(num_requests).collect::<Vec<_>>();
                cycle
            })
            .collect::<Vec<_>>();

        let cycles: Vec<_> = ramp
            .into_iter()
            .map(|cycle| join_all(cycle.into_iter().map(|req| req)))
            .collect::<Vec<_>>();

        stream::iter(cycles)
            .then(|cycle| async move {
                let cycle_results = cycle.await;
                time::sleep(Duration::from_millis(100)).await;
                cycle_results
                    .into_iter()
                    .map(|res| res.clone())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .await;

        Ok(())
    }

    pub async fn run(&self, printer: &Printer) -> Result<Loads> {
        let mut loads: Loads = vec![];
        for endpoint in &self.config.endpoints {
            let should_ramp = match endpoint.ramp {
                Some(false) => false,
                _ => true,
            };
            if should_ramp {
                printer.print_with_yellow("Warming", &format!("up {} with a logarithmic ramp", endpoint.name), 4);
                self.log_ramp(&endpoint).await?;
                printer.clear_previous()
                    .print_with_green("Warmed", &format!("up {} with a logarithmic ramp", endpoint.name), 4);
                printer.print_with_yellow("Running", &format!("load for {}", endpoint.name), 4);
            }
            let num_cycles = endpoint.cycles.clone();
            let num_concurrent_requests = endpoint.concurrent_requests.clone();

            let raw_cycles = (0..num_cycles)
                .map(|_| self.make_cycle(&endpoint).unwrap())
                .collect::<Vec<_>>();

            let cycles: Vec<_> = raw_cycles
                .into_iter()
                .map(|cycle| join_all(cycle.into_iter().map(|req| req)))
                .collect::<Vec<_>>();

            let results = stream::iter(cycles)
                .then(|cycle| async move {
                    let cycle_results = cycle.await;
                    time::sleep(Duration::from_millis(100)).await;
                    cycle_results
                        .into_iter()
                        .map(|res| res.clone())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
                .await;

            loads.push(SingleLoad {
                cycles: results,
                num_cycles,
                num_concurrent_requests,
                endpoint_name: endpoint.name.clone(),
                endpoint_url: endpoint.url.clone(),
            });

            printer.clear_previous();
            printer.print_with_green("Finished", &format!("load for {}", endpoint.name), 4);
        }

        Ok(loads)
    }
}
