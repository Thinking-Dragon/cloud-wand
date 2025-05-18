use anyhow::Result;
use cloud_wand_parser::{deployment_definition::DeploymentTarget, package_definition::PackageDefinition};

pub struct CreateAction {
    pub package: PackageDefinition,
    pub target: DeploymentTarget,
}

impl CreateAction {
    pub fn execute(&self) -> Result<()> {
        Ok(())
    }
}
