mod compare;
mod config;
mod printer;
mod process;
mod request;
mod runner;
mod snapshot;
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use compare::compare_tests;
use config::Config;
use console::Term;
use printer::Printer;
use process::process;
use runner::Runner;
use snapshot::Snapshot;
use std::fs;
use tokio;

#[derive(Parser)]
struct Args {
    #[arg(long = "no-snapshot", required = false)]
    no_snapshot: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let term = Term::stdout();
    let printer = Printer::new(term);

    match fs::metadata("./ballast.json") {
        Ok(metadata) => metadata,
        Err(_) => {
            printer
                .print_with_red(
                    "ERROR",
                    "No ballast.json file found in current directory (https://github.com/synoet/ballast/blob/main/README.md)",
                    0,
                );
            return Ok(());
        }
    };

    let config: Config = Config::from_config_file("./ballast.json")
        .context("Failed to read ./ballast.json config file")?;
    printer.print_with_green(
        "Loaded",
        &format!(
            "config with {} tests from ./ballast.json",
            config.endpoints.len()
        ),
        0,
    );
    let runner = Runner::new(config.clone());
    let matches = Args::parse();
    let results = runner.run(&printer).await?;
    let latest_snapshot = Snapshot::latest()?;
    printer.blank_line().print_with_yellow(
        "Processing",
        &format!("{} tests", config.endpoints.len()),
        0,
    );
    let processed_tests = process(&results, &config, latest_snapshot.as_ref());
    printer.clear_previous().print_with_green(
        "Processed",
        &format!("{} tests", config.endpoints.len()),
        0,
    );

    compare_tests(
        &processed_tests,
        &config,
        latest_snapshot.as_ref(),
        &printer,
    );

    if !matches.no_snapshot {
        Snapshot::new(processed_tests.clone()).unwrap().write()?;
        printer.blank_line().print_with_green(
            "Saved",
            &format!(
                "snapshot with {} tests to {}",
                processed_tests.len(),
                "./.ballast_snapshot.json"
            ),
            0,
        );
    }

    Ok(())
}
