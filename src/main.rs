use std::process::ExitCode;

use clap::Parser;
use forge_cli::cli::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let report = forge_cli::run(cli);

    print!("{}", report.render());

    ExitCode::from(report.exit_code())
}
