use anyhow::Result;
use cloud_wand_parser::{deployment_definition::DeploymentTarget, package_definition::PackageDefinition};

pub struct TeardownAction {
    pub package: PackageDefinition,
    pub target: DeploymentTarget,
}

impl TeardownAction {
    pub fn execute(&self) -> Result<()> {
        Ok(())
    }
}
