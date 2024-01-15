use crate::config::{EndpointConfig, Config};
use crate::process::Test;
use crate::printer::Printer;
use crate::snapshot::Snapshot;

pub fn compare_tests(tests: &Vec<Test>, config: &Config, latest: Option<&Snapshot>, printer: &Printer) {
    for test in tests {
        let other = match latest {
            Some(snapshot) => snapshot.tests.iter().find(|t| t.config.endpoint_name == test.config.endpoint_name),
            None => None,
        };
        let endpoint_config: &EndpointConfig = config.endpoints.iter().find(|e| e.name == test.config.endpoint_name).unwrap();
        match test.success {

            true => {
                printer
                    .blank_line()
                    .print_with_green(
                        "PASS",
                        &format!("{} {}", test.config.endpoint_name, test.config.endpoint_url),
                        4,
                    );
            },
            false => {
                let mut reasons_for_failure: Vec<String> = vec![];
                printer
                    .blank_line()
                    .print_with_red(
                        "FAIL",
                        &format!("{} {}", test.config.endpoint_name, test.config.endpoint_url),
                        4,
                    );

                if !test.within_threshold {
                    match other {
                        Some(other) => {
                            let expected = other.stats.average_response_time;
                            let actual = test.stats.average_response_time;
                            printer.print_with_yellow(
                                "Threshold",
                                &format!(
                                    "average response time {}ms (expected {}ms +/- {}ms)",
                                    actual, expected, endpoint_config.threshold.unwrap()
                                ),
                                8
                            );
                        },
                        None => {}
                    }
                };

                if test.expected.status_code == Some(false){
                    reasons_for_failure.push(format!(
                        "expected status code {}",
                        endpoint_config.expected_status.unwrap()
                    ));
                    printer.print_with_yellow(
                        "Expected",
                        &format!(
                            "expected status code {}",
                            endpoint_config.expected_status.unwrap(),
                        ),
                        8
                    );
                }

                if test.expected.body == Some(false){
                    reasons_for_failure.push(format!(
                        "expected body {}",
                        endpoint_config.expected_body.as_ref().unwrap()
                    ));
                    printer.print_with_yellow(
                        "Expected",
                        &format!(
                            "expected body {:?}",
                            endpoint_config.expected_body.as_ref().unwrap(),
                        ),
                        8
                    );
                }

                if test.expected.headers == Some(false){
                    printer.print_with_yellow(
                        "Expected",
                        &format!(
                            "expected headers {:?}",
                            endpoint_config.expected_headers.as_ref().unwrap(),
                        ),
                        8
                    );
                }


            },
        };

    }


}
