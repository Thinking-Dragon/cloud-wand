use anyhow::Result;
use clap::Parser;
use command::{apply::{apply_command, ApplyArgs}, teardown::{teardown_command, TeardownArgs}};

mod command;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
enum Cli {
    Apply(ApplyArgs),
    Teardown(TeardownArgs),
}

fn main() -> Result<()> {
    match Cli::parse() {
        Cli::Apply(args) => apply_command(args),
        Cli::Teardown(args) => teardown_command(args),
    }
}
