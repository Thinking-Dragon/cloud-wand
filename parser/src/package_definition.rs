use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDefinition {
    pub name: String,
    pub resources: Vec<PackageResource>,
    pub create: String,
    pub update: String,
    pub teardown: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageResource {
    File(FileResource),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResource {
    pub file: String,
    pub origin: String,
    pub location: String,
}
