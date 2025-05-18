use anyhow::{Ok, Result};

#[derive(Debug, Clone)]
pub struct HostManipulator {
    pub url: String,
    pub user: String,
}

impl HostManipulator {
    pub fn connect(url: String, user: String) -> Result<Self> {
        Ok(
            HostManipulator { url, user }
        )
    }

    pub fn execute(&self, command: String) -> Result<()> {
        Ok(())
    }

    pub fn package_exists(&self, package_name: &String) -> bool {
        true
    }
}
