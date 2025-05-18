use anyhow::Result;
use cloud_wand_parser::{deployment_definition::DeploymentTarget, package_definition::PackageDefinition};

pub struct UpdateAction {
    pub package: PackageDefinition,
    pub target: DeploymentTarget,
}

impl UpdateAction {
    pub fn execute(&self) -> Result<()> {
        Ok(())
    }
}
