use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentDefinition {
    pub name: String,
    pub hosts: Vec<Host>,
    pub deployment: Vec<DeploymentTarget>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub name: String,
    pub url: String,
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeploymentTarget {
    Package(PackageDeploymentTarget),
    Cluster(ClusterDeploymentTarget),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDeploymentTarget {
    pub package: String,
    pub hosts: Vec<String>,
    pub config: Option<HashMap<String, ConfigValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterDeploymentTarget {
    pub cluster: String,
    pub hosts: Vec<String>,
    pub config: Option<HashMap<String, ConfigValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue {
    String(String),
    Unsigned32(u32),
}
