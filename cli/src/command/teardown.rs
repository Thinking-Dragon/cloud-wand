use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;


#[derive(Debug, Parser)]
pub struct TeardownArgs {
    #[arg(short, long)]
    project: Option<PathBuf>,
}

pub fn teardown_command(_args: TeardownArgs) -> Result<()> {
    Ok(())
}
