use anyhow::{bail, Result};
use cloud_wand_parser::{deployment::Deployment, deployment_definition::DeploymentTarget};

use crate::orchestration_context::OrchestrationContext;

#[tokio::main]
pub async fn orchestrate_apply(deployment: Deployment) -> Result<()> {
    let ctx: OrchestrationContext = OrchestrationContext::from_deployment(deployment)?;

    for target in ctx.targets {
        match target {
            DeploymentTarget::Package(ref target) => {
                for host in &target.hosts {
                    if let Some(host) = ctx.hosts.get(host) {
                        if let Some(package) = ctx.packages.get(&target.package) {
                            if host.package_exists(&target.package) {
                                println!("Creating {} on {}", target.package, host.url);
                                host.execute(package.create.clone())?;
                            }
                            else {
                                println!("Updating {} on {}", target.package, host.url);
                                host.execute(package.update.clone())?;
                            }
                        }
                        else {
                            bail!("Could not find definition for package {}", target.package);
                        }
                    }
                    else {
                        bail!("Could not find host {} when trying to apply package {}", host, target.package);
                    }
                }
            },
            DeploymentTarget::Cluster(_target) => {

            },
        }
    }

    Ok(())
}
