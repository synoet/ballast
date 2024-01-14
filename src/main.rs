mod config;
mod process;
mod request;
mod runner;
mod snapshot;
mod display;
use anyhow::Result;
use console::Term;
use clap::Parser;
use config::Config;
use tokio;

use process::process;
use runner::Runner;
use snapshot::Snapshot;
use display::Display;

#[derive(Parser)]
struct Args {
    #[arg(long = "no-snapshot", required = false)]
    no_snapshot: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let term = Term::stdout();
    let config: Config = Config::from_config_file("./ballast.json")?;
    let runner = Runner::new(config.clone());
    let matches = Args::parse();
    let display = Display::new(term);

    let results = runner.run().await?;
    let latest_snapshot = Snapshot::latest()?;
    let processed_tests = process(&results, &config, latest_snapshot.as_ref());

    display.print_tests(&processed_tests);

    if !matches.no_snapshot {
        Snapshot::new(processed_tests.clone()).unwrap().write()?;
    }

    Ok(())
}
