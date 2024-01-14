mod config;
mod output;
mod process;
mod request;
mod runner;
mod snapshot;
mod ui;
use anyhow::Result;
use clap::Parser;
use config::Config;
use tokio;

use process::process;
use runner::Runner;
use snapshot::Snapshot;

#[derive(Parser)]
struct Args {
    #[arg(long = "no-snapshot", required = false)]
    no_snapshot: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = Config::from_config_file("./ballast.json")?;
    let runner = runner::Runner::new(config.clone());

    let results = runner.run().await?;
    let latest_snapshot = Snapshot::latest()?;
    let processed_tests = process::process(&results, &config, latest_snapshot.as_ref());

    dbg!(&processed_tests);
    Ok(())
}
