use std::{fs, path::PathBuf};

use anyhow::{bail, Result};

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
            println!("{:#?}", directory);
            bail!("Deployment path must be a directory")
        }

        let mut deployment_definitions: Vec<DeploymentDefinition> = Vec::new();
        let mut package_definitions: Vec<PackageDefinition> = Vec::new();

        for entry in fs::read_dir(directory)? {
            let entry: PathBuf = entry?.path();
            if entry.is_file() {
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
        }

        Ok(
            Deployment::new(
                deployment_definitions,
                package_definitions,
            )
        )
    }
}
