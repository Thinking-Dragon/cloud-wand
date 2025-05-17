use std::{fs::File, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    deployment_definition::DeploymentDefinition,
    package_definition::PackageDefinition
};


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnyDefinition {
    Deployment(DeploymentDefinition),
    Package(PackageDefinition),
}

impl AnyDefinition {
    pub fn parse(path: PathBuf) -> Result<Self> {
        let raw_definition: File = File::open(path)?;
        let definition: AnyDefinition = serde_yaml::from_reader(raw_definition)?;

        Ok(definition)
    }
}
