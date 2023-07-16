use crate::request::RequestOutput;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Output {
    pub endpoint_name: String,
    pub endpoint_url: String,
    pub cycles: u64,
    pub concurrent_requests: u64,
    pub average_response_time: f64,
    pub min_response_time: f64,
    pub max_response_time: f64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct OutputToProcess {
    pub output: RequestOutput,
    pub endpoint_name: String,
    pub endpoint_url: String,
    pub cycles: u64,
    pub concurrent_requests: u64,
}

pub fn parse_results(results: Vec<Vec<Vec<OutputToProcess>>>) -> Result<Vec<Output>> {
    let mut output: Vec<Output> = Vec::new();
    for res in results.iter() {
        let average_response_time = res
            .iter()
            .map(|x| x.iter().map(|y| y.output.duration as f64).sum::<f64>() / x.len() as f64)
            .sum::<f64>()
            / res.len() as f64;
        let min_response_time = res
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| y.output.duration as f64)
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_response_time = res
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| y.output.duration as f64)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        output.push(Output {
            endpoint_name: res[0][0].endpoint_name.clone(),
            endpoint_url: res[0][0].endpoint_url.clone(),
            cycles: res.len() as u64,
            concurrent_requests: res[0].len() as u64,
            average_response_time,
            min_response_time,
            max_response_time,
        })
    }
    Ok(output)
}
