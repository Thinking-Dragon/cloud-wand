use std::path::PathBuf;

use anyhow::{bail, Result};
use walkdir::WalkDir;

use crate::{any_definition::AnyDefinition, deployment_definition::DeploymentDefinition, package_definition::PackageDefinition};

#[derive(Debug)]
pub struct Deployment {
    pub deployment_definitions: Vec<DeploymentDefinition>,
    pub package_definitions: Vec<PackageDefinition>,
}

impl Deployment {
    pub fn new(
        deployment_definitions: Vec<DeploymentDefinition>,
        package_definitions: Vec<PackageDefinition>,
    ) -> Self {
        Deployment { deployment_definitions, package_definitions }
    }

    pub fn from_directory(directory: PathBuf) -> Result<Self> {
        if !directory.is_dir() {
            bail!("Deployment path must be a directory")
        }

        let mut deployment_definitions: Vec<DeploymentDefinition> = Vec::new();
        let mut package_definitions: Vec<PackageDefinition> = Vec::new();

        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let entry: PathBuf = PathBuf::from(entry.path());
            if let Some(extension) = entry.extension() {
                if extension == "yaml" {
                    match AnyDefinition::parse(entry)? {
                        AnyDefinition::Deployment(definition) => {
                            deployment_definitions.push(definition);
                        },
                        AnyDefinition::Package(definition) => {
                            package_definitions.push(definition);
                        },
                    }
                }
            }
        }

        Ok(
            Deployment::new(
                deployment_definitions,
                package_definitions,
            )
        )
    }
}
