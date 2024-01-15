use anyhow::Result;
use futures::future::join_all;
use std::vec;

use crate::config::Config;
use crate::request::{RequestOutput, TimedRequest};
use crate::printer::Printer;

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

    pub async fn run(&self, printer: &Printer) -> Result<Loads> {
        let mut loads: Loads = vec![];
        for endpoint in &self.config.endpoints {
            printer.print_with_yellow(
                "Running",
                &format!("load for {}", endpoint.name),
                4,
            );
            let num_cycles = endpoint.cycles.clone();
            let num_concurrent_requests = endpoint.concurrent_requests.clone();

            let make_request = || {
                TimedRequest::from_config(&self.client, &endpoint)
                    .unwrap()
                    .request
            };

            let make_cycle = || {
                (0..num_concurrent_requests)
                    .map(|_| make_request())
                    .collect::<Vec<_>>()
            };

            let raw_cycles = (0..num_cycles).map(|_| make_cycle()).collect::<Vec<_>>();

            let cycles: Vec<_> = raw_cycles
                .into_iter()
                .map(|cycle| join_all(cycle.into_iter().map(|req| req)))
                .collect::<Vec<_>>();

            let results = join_all(cycles.into_iter().map(|cycle| cycle))
                .await
                .into_iter()
                .map(|cycle_results| {
                    cycle_results
                        .into_iter()
                        .map(|res| res.clone())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            loads.push(SingleLoad {
                cycles: results,
                num_cycles,
                num_concurrent_requests,
                endpoint_name: endpoint.name.clone(),
                endpoint_url: endpoint.url.clone(),
            });

            printer.clear_previous();
            printer.print_with_green(
                "Finished",
                &format!("load for {}", endpoint.name),
                4,
            );
        }

        Ok(loads)
    }
}
