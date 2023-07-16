mod config;
mod output;
mod request;
mod snapshot;
mod ui;
use config::Config;
use console::{Style, Term};
use futures::future::join_all;
use output::{parse_results, OutputToProcess};
use request::TimedRequest;
use snapshot::Snapshot;
use tokio;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Term::stdout();
    let yellow = Style::new().yellow().bold();
    let green = Style::new().green().bold();
    let white = Style::new().white().bold();
    let config: Config = Config::from_config_file("./ballast.toml".to_string())?;

    term.write_line(&format!(
        "{} configuration file from {}",
        green.apply_to("Read"),
        white.apply_to("./ballast.toml")
    ))
    .ok();

    let mut all_results: Vec<Vec<Vec<OutputToProcess>>> = Vec::new();
    for endpoint in config.endpoints {
        term.write_line(&format!(
            "{} loads for endpoint {}",
            green.apply_to("Starting"),
            white.apply_to(endpoint.name.clone())
        ))
        .ok();

        let mut endpoint_results: Vec<Vec<OutputToProcess>> = Vec::new();
        for idx in 0..endpoint.cycles {
            let mut requests_in_cycle = Vec::new();
            let endpoint_name = endpoint.name.clone();
            let endpoint_url = endpoint.url.clone();

            for _ in 0..endpoint.concurrent_requests {
                let request = TimedRequest::from_config(&endpoint).await.request;
                requests_in_cycle.push(request);
            }

            if idx < endpoint.cycles - 1 {
                ui::print_endpoint_in_progress(
                    &term,
                    &endpoint.url,
                    endpoint.cycles,
                    idx
                )
            }
            if idx == endpoint.cycles - 1 {
                ui::print_endpoint_finished(
                    &term,
                    endpoint.cycles,
                    &endpoint.url,
                )
            }

            endpoint_results.push(
                join_all(requests_in_cycle.into_iter().map(move |x| {
                    let endpoint_name = endpoint_name.clone();
                    let endpoint_url = endpoint_url.clone();
                    let cycles = endpoint.cycles;
                    let concurrent_requests = endpoint.concurrent_requests;
                    async move {
                        OutputToProcess {
                            output: x.await,
                            endpoint_name,
                            endpoint_url,
                            cycles,
                            concurrent_requests,
                        }
                    }
                }))
                .await,
            );
        }
        all_results.push(endpoint_results);
    }

    let outputs = parse_results(all_results);
    let latest = Snapshot::latest();

    term.write_line("").ok();
    term.write_line(&format!("{}\n", green.apply_to("Results:")))
        .ok();

    for output in outputs.iter() {
        let corresponding_latest = latest
            .as_ref()
            .unwrap()
            .outputs
            .iter()
            .find(|x| x.endpoint_name == output.endpoint_name)
            .unwrap();

        ui::print_time_title(&term, &output.endpoint_name, &output.endpoint_url);
        ui::print_time_stat(&term, &(output.cycles as f64), None, "Cycles");
        ui::print_time_stat(
            &term,
            &(output.concurrent_requests as f64),
            None,
            "concurrent requests",
        );
        ui::print_time_stat(
            &term,
            &(output.average_response_time),
            Some(&(corresponding_latest.average_response_time)),
            "Average Response Time",
        );
        ui::print_time_stat(
            &term,
            &(output.min_response_time),
            Some(&(corresponding_latest.min_response_time)),
            "Min Response Time",
        );
        ui::print_time_stat(
            &term,
            &(output.max_response_time),
            Some(&(corresponding_latest.max_response_time)),
            "Max Response Time",
        );
        term.write_line("").ok();
    }

    term.write_line(&format!("{} to snapshot file", yellow.apply_to("Writing")))
        .ok();
    let snapshot = Snapshot::new(outputs);
    snapshot.write();
    term.write_line(&format!(
        "{} to snapshot file (./.ballast_snapshot.json)",
        green.apply_to("Wrote")
    ))
    .ok();
    Ok(())
}
