use clap::Parser;
use futures::future::join_all;
use reqwest::Client;
use std::time::Instant;
use tokio;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    url: String,
    #[arg(long, required = false, default_value = "1")]
    concurrent: usize,
    #[arg(long, required = false, default_value = "1")]
    cycles: usize,
}

struct RequestOutput {
    duration: u128,
    success: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();


    for idx in 0..args.cycles {
        let mut requests = Vec::new();

        for _ in 0..args.concurrent {
            let client = Client::new();
            let url = args.url.to_string();

            let request = async move {
                let start = Instant::now();
                let result = client.get(&url).send().await;
                let duration = start.elapsed();
                RequestOutput {
                    duration: duration.as_millis(),
                    success: result.is_ok(),
                }
            };

            requests.push(request);
        }

        let results = join_all(requests).await;

        let total = results.iter().fold(0, |acc, x| acc + x.duration);
        let average = total / args.concurrent as u128;
        let success = results.iter().filter(|x| x.success).count();
        let failed = results.iter().filter(|x| !x.success).count();

        println!("cycle {} :: average {}ms :: success {} :: failed {}", idx + 1, average, success, failed);
        println!("average: {}ms", average);
        println!("success: {}", success);
        println!("failed: {}", failed);
    }

    Ok(())
}
