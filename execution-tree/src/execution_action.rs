use anyhow::Result;

use crate::{apply_action::ApplyAction, teardown_action::TeardownAction};

pub enum ExecutionAction {
    Apply(ApplyAction),
    Teardown(TeardownAction),
}

impl ExecutionAction {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Apply(action) => action.execute(),
            Self::Teardown(action) => action.execute(),
        }
    }
}
