mod config;
mod output;
mod request;
mod runner;
mod snapshot;
mod ui;
use anyhow::Result;
use clap::Parser;
use config::Config;
use tokio;

#[derive(Parser)]
struct Args {
    #[arg(long = "no-snapshot", required = false)]
    no_snapshot: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = Config::from_config_file("./ballast.json")?;
    let runner = runner::Runner::new(config);

    let results = runner.run().await?;
    dbg!(&results);
    Ok(())
}
