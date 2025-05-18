use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use cloud_wand_orchestrator::orchestrate_teardown::orchestrate_teardown;
use cloud_wand_parser::deployment::Deployment;

#[derive(Debug, Parser)]
pub struct TeardownArgs {
    #[arg(short, long)]
    deployment: Option<PathBuf>,
}

pub fn teardown_command(args: TeardownArgs) -> Result<()> {
    let deployment_path: PathBuf = match args.deployment {
        Some(deployment) => deployment,
        None => std::env::current_dir()?,
    };

    let deployment: Deployment = Deployment::from_directory(deployment_path)?;
    orchestrate_teardown(deployment)
}
