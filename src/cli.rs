use clap::{Command, arg};
pub fn cli() -> Command {
    Command::new("ballast")
        .subcommand_required(false)
        .about("A simple cli tool to run snapshot performance tests incrementally against local apis")
        .flag(arg!("--no-snapshot"))
        .arg(arg!("--desc <desc>"))
}
