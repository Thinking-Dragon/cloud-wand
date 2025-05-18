use anyhow::Result;
use cloud_wand_parser::deployment::Deployment;

#[tokio::main]
pub async fn orchestrate_teardown(_deployment: Deployment) -> Result<()> {
    Ok(())
}
