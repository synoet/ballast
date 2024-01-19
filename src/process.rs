use crate::config::{Config, EndpointConfig};
use crate::runner::Loads;
use crate::snapshot::Snapshot;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoadStats {
    pub average_response_time: f64,
    pub min_response_time: f64,
    pub max_response_time: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expected {
    pub body: Option<bool>,
    pub status_code: Option<bool>,
    pub headers: Option<bool>,
}

impl Expected {
    pub fn passes(&self) -> bool {
        let is_truthy = |e: Option<bool>| match e {
            Some(e) => e,
            None => true,
        };

        is_truthy(self.body) && is_truthy(self.status_code) && is_truthy(self.headers)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimpleConfig {
    pub num_cycles: u64,
    pub num_concurrent_requests: u64,
    pub endpoint_name: String,
    pub endpoint_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Test {
    pub success: bool,
    pub within_threshold: bool,
    pub expected: Expected,
    pub stats: LoadStats,
    pub config: SimpleConfig,
}
type Expectations = Vec<Expected>;

trait ReduceExpectations {
    fn reduce_expectations(&self, endpoint: &EndpointConfig) -> Expected;
}

impl ReduceExpectations for Expectations {
    fn reduce_expectations(&self, endpoint: &EndpointConfig) -> Expected {
        Expected {
            body: match endpoint.expected_body {
                Some(_) => Some(self.iter().all(|e| e.body == Some(true))),
                None => None,
            },
            status_code: match endpoint.expected_status {
                Some(_) => Some(self.iter().all(|e| e.status_code == Some(true))),
                None => None,
            },
            headers: match endpoint.expected_headers {
                Some(_) => Some(self.iter().all(|e| e.headers == Some(true))),
                None => None,
            },
        }
    }
}

pub fn process(loads: &Loads, config: &Config, snapshot: Option<&Snapshot>) -> Vec<Test> {
    let mut tests: Vec<Test> = vec![];
    for load in loads {
        let endpoint = config
            .endpoints
            .iter()
            .find(|e| e.name == load.endpoint_name)
            .unwrap();

        let mut cycles_expected = vec![];

        for cycle in load.cycles.iter() {
            let mut response_expected: Vec<Expected> = vec![];
            for request in cycle.iter() {
                let does_status_match = match endpoint.expected_status {
                    Some(status) => Some(status == request.status),
                    None => None,
                };

                let does_body_match = match [
                    endpoint.expected_body.clone(),
                    request.response_body.clone(),
                ] {
                    [None, None] => None,
                    [Some(_), None] => Some(false),
                    [None, Some(_)] => None,
                    [Some(expected), Some(actual)] => Some(expected == actual),
                };

                let does_headers_match = match [
                    endpoint.expected_headers.clone(),
                    request.response_headers.clone(),
                ] {
                    [None, None] => None,
                    [Some(_), None] => Some(false),
                    [None, Some(_)] => None,
                    [Some(expected), Some(actual)] => Some(expected == actual),
                };

                // Does this request match the expected tests?
                let expected = Expected {
                    body: does_body_match,
                    status_code: does_status_match,
                    headers: does_headers_match,
                };

                response_expected.push(expected);
            }

            // Here we check wether an individual cycle matches expected tests?
            cycles_expected.push(response_expected.reduce_expectations(endpoint));
        }

        let expected = cycles_expected.reduce_expectations(endpoint);

        let average_response_time = load
            .cycles
            .iter()
            .map(|c| c.iter().map(|r| r.duration).sum::<u128>() / c.len() as u128)
            .sum::<u128>()
            / load.cycles.len() as u128;
        let max_response_time = load
            .cycles
            .iter()
            .map(|c| c.iter().map(|r| r.duration).max().unwrap())
            .max()
            .unwrap();
        let min_response_time = load
            .cycles
            .iter()
            .map(|c| c.iter().map(|r| r.duration).min().unwrap())
            .min()
            .unwrap();

        let stats = LoadStats {
            average_response_time: average_response_time as f64,
            max_response_time: max_response_time as f64,
            min_response_time: min_response_time as f64,
        };

        let test = match snapshot {
            Some(snapshot) => {
                let latest_test = snapshot
                    .tests
                    .iter()
                    .find(|t| t.config.endpoint_name == load.endpoint_name)
                    .unwrap();
                let within_threshold = stats.average_response_time
                    < latest_test.stats.average_response_time
                        + endpoint.threshold.unwrap_or(250) as f64;

                Test {
                    success: expected.passes() && within_threshold,
                    within_threshold,
                    expected,
                    stats,
                    config: SimpleConfig {
                        num_cycles: load.cycles.len() as u64,
                        num_concurrent_requests: load.cycles[0].len() as u64,
                        endpoint_name: load.endpoint_name.clone(),
                        endpoint_url: endpoint.url.clone(),
                    },
                }
            }
            None => Test {
                success: expected.passes(),
                within_threshold: true,
                expected,
                stats,
                config: SimpleConfig {
                    num_cycles: load.cycles.len() as u64,
                    num_concurrent_requests: load.cycles[0].len() as u64,
                    endpoint_name: load.endpoint_name.clone(),
                    endpoint_url: endpoint.url.clone(),
                },
            },
        };

        tests.push(test);
    }

    tests
}
