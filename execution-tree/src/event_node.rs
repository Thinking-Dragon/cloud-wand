use std::{collections::HashMap, sync::Arc};

use anyhow::{bail, Result};
use cloud_wand_host_manipulator::host_manipulator::HostManipulator;
use cloud_wand_parser::{deployment::Deployment, deployment_definition::{DeploymentTarget, PackageDeploymentTarget}, package_definition::PackageDefinition};

use crate::{apply_action::ApplyAction, create_action::CreateAction, execution_action::ExecutionAction};

pub struct ExecutionNode {
    pub action: ExecutionAction,
    pub branches: Vec<ExecutionNode>,
}

impl ExecutionNode {
    pub fn from_deployment(deployment: Deployment) -> Result<Vec<ExecutionNode>> {
        let mut packages: HashMap<String, PackageDefinition> = HashMap::new();
        for package in &deployment.package_definitions {
            packages.insert(package.name.clone(), package.clone());
        }

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

        let mut nodes: Vec<ExecutionNode> = Vec::new();
        for target in targets {
            match target {
                DeploymentTarget::Package(target) => {
                    if let Some(package) = packages.get(&target.package) {
                        for host in &target.hosts {
                            let target_host = DeploymentTarget::Package(
                                PackageDeploymentTarget {
                                    hosts: vec![host.clone()],
                                    ..target.clone()
                                }
                            );

                            nodes.push(
                                ExecutionNode {
                                    action: ExecutionAction::Apply(
                                        ApplyAction::Create(
                                            CreateAction {
                                                package: package.clone(),
                                                target: target_host,
                                            }
                                        )
                                    ),
                                    branches: Vec::new(),
                                }
                            );
                        }
                    }
                },
                DeploymentTarget::Cluster(_target) => {},
            }
        }

        Ok(Vec::new())
    }

    pub fn execute(&self) -> Result<()> {
        self.action.execute()
    }
}
