use futures::future::join_all;
use anyhow::Result;
use std::vec;

use crate::config::Config;
use crate::request::{RequestOutput, TimedRequest};

pub struct Runner {
    config: Config,
    client: reqwest::Client,
}

type SingleCycle = Vec<RequestOutput>;

#[derive(Debug)]
pub struct SingleTest {
    pub cycles: Vec<SingleCycle>,
    pub num_cycles: u64,
    pub num_concurrent_requests: u64,
    pub endpoint_name: String,
    pub endpoint_url: String,
}

impl Runner {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::new();
        Self { config, client }
    }

    pub async fn run(&self) -> Result<Vec<SingleTest>> {
        let mut all_tests: Vec<SingleTest> = vec![];
        for endpoint in &self.config.endpoints {
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

            let results = join_all(cycles)
                .await
                .into_iter()
                .map(|cycle_results| {
                    cycle_results
                        .into_iter()
                        .map(|res| res.clone())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            all_tests.push(SingleTest {
                cycles: results,
                num_cycles,
                num_concurrent_requests,
                endpoint_name: endpoint.name.clone(),
                endpoint_url: endpoint.url.clone(),
            })
        }

        Ok(all_tests)
    }
}
