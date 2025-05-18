use std::{collections::HashMap, sync::Arc};

use anyhow::{bail, Result};
use cloud_wand_host_manipulator::host_manipulator::HostManipulator;
use cloud_wand_parser::{deployment::Deployment, deployment_definition::DeploymentTarget, package_definition::PackageDefinition};

pub struct OrchestrationContext {
    pub hosts: HashMap<String, Arc<HostManipulator>>,
    pub packages: HashMap<String, PackageDefinition>,
    pub targets: Vec<DeploymentTarget>,
}

impl OrchestrationContext {
    pub fn from_deployment(deployment: Deployment) -> Result<Self> {
        let mut hosts: HashMap<String, Arc<HostManipulator>> = HashMap::new();
        let mut targets: Vec<DeploymentTarget> = Vec::new();
        for definition in &deployment.deployment_definitions {
            for host in &definition.hosts {
                if hosts.contains_key(&host.name) {
                    bail!("Duplicate host name: {}", host.name)
                }
                hosts.insert(
                    host.name.clone(),
                    Arc::new(
                        HostManipulator::connect(
                            host.url.clone(),
                            host.user.clone()
                        )?
                    )
                );
            }
            targets.extend(definition.deployment.clone());
        }

        let mut packages: HashMap<String, PackageDefinition> = HashMap::new();
        for package in &deployment.package_definitions {
            packages.insert(package.name.clone(), package.clone());
        }

        Ok(OrchestrationContext { hosts, packages, targets })
    }
}
